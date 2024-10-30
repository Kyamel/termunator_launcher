// world.rs

// --------------------------------------------------------------------------------------------------------------------------------
// The World (or sometimes referred to as "Scene" or "Context") is the
// overarching manager that contains all entities, components, and systems.
// It maintains mappings of which entities have which components and is responsible for updating each system in a defined order.
// The World acts as the central hub where systems interact with entities and components,
// ensuring they communicate and modify the game state correctly.
// -------------------------------------------------------------------------------------------------------------------------------

use crossterm::{cursor::{Hide, MoveTo, Show}, execute};
use crossterm::{
    event::{self},
    queue,
    terminal::{self},
};

use std::io::{stdout, Write};

use std::collections::{HashMap, HashSet};
use std::any::TypeId;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::Entity::Entity;
use crate::Components::IComponent;
use crate::Systems::ISystem;



pub struct World {
    next_entity_id: u32,
    systems: HashMap<String, Box<dyn ISystem>>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn IComponent>>>,
    entities: HashSet<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entities: HashSet::new(),
            components: HashMap::new(),
            systems: HashMap::new(),
            // Variável para FPS

        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.insert(entity);
        entity
    }

    pub fn add_component<T: IComponent>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert(HashMap::new())
            .insert(entity, Box::new(component));
    }

    pub fn add_system<S: ISystem + 'static>(&mut self, name: &str, system: S) {
        self.systems.entry(name.to_owned()).or_insert(Box::new(system));
    }
}

// Query components
impl World {
    pub fn get_component<T: IComponent>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|entity_map| entity_map.get(&entity))
            .and_then(|boxed_component| boxed_component.as_any().downcast_ref::<T>())
    }

    pub fn get_component_mut<T: IComponent>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>()) // Acessa o HashMap de componentes do tipo T
            .and_then(|entity_map| entity_map.get_mut(&entity)) // Obtém o componente específico da entidade
            .and_then(|boxed_component| boxed_component.as_any_mut().downcast_mut::<T>()) // Realiza o downcast para &mut T
    }

    pub fn query_components(&self, component_types: &[TypeId]) -> Vec<(Entity, Vec<&dyn IComponent>)> {
        let mut results = Vec::new();

        if let Some((first_type, remaining_types)) = component_types.split_first() {
            if let Some(base_components) = self.components.get(first_type) {
                'entity_loop: for (entity, base_component) in base_components {
                    let mut entity_components = vec![base_component.as_ref() as &dyn IComponent];

                    for component_type in remaining_types {
                        if let Some(component_map) = self.components.get(component_type) {
                            if let Some(component) = component_map.get(entity) {
                                entity_components.push(component.as_ref() as &dyn IComponent);
                            } else {
                                continue 'entity_loop;
                            }
                        } else {
                            continue 'entity_loop;
                        }
                    }
                    results.push((*entity, entity_components));
                }
            }
        }
        results
    }

    pub fn query_component<T: IComponent>(&self) -> Vec<(Entity, &T)> {
        let comp = self.components.get(&TypeId::of::<T>());

        if let Some(comp) = comp {
            comp.iter()
                .map(|(entity, component)| {
                    (
                        *entity,
                        component.as_any().downcast_ref::<T>().unwrap(),
                    )
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

// Remove
impl World {
    pub fn remove_component<T: IComponent>(&mut self, entity: Entity) {
        if let Some(components) = self.components.get_mut(&TypeId::of::<T>()) {
            components.remove(&entity);
        }
    }

    pub fn remove_system<S: ISystem + 'static>(&mut self, name: String) {
        self.systems.remove(&name);
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
        for components in self.components.values_mut() {
            components.remove(&entity);
        }
    }
}

impl World {
    pub fn update(&mut self, delta_time: u64) {

        let frame_duration = Duration::from_millis(1000 / delta_time);
        let start_time = Instant::now();
        let mut stdout = stdout();

        // Limpeza e atualização - aqui você adiciona a lógica do jogo e o redesenho
        queue!(
            stdout,
            MoveTo(0, 0),
        ).expect("Erro ao enfileirar comandos");


        for system in self.systems.values_mut() {
            system.update(&mut self.components);
        }

        stdout.flush().expect("Erro ao atualizar terminal");

        // Limpa todos os eventos pendentes para evitar o acúmulo de inputs
        while event::poll(Duration::from_millis(0)).unwrap() {
            event::read().unwrap(); // Lê e descarta o evento
        }

        // Controle de FPS
        let elapsed = start_time.elapsed();
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }

    }
}


impl World {
    pub fn init(width: u16, height: u16) -> Result<(u16, u16), Box<dyn std::error::Error>> {
        // Configuração inicial
        terminal::enable_raw_mode().map_err(|e| {
            eprintln!("Erro ao ativar o modo raw: {:?}", e);
            e
        })?;
    
        // Obtém o tamanho atual do terminal
        let (max_cols, max_rows) = terminal::size().map_err(|e| {
            eprintln!("Erro ao obter tamanho do console: {:?}", e);
            e
        })?;
        
        let mut window = (width, height);
    
        // Verifica se o tamanho informado está dentro dos limites
        if width > max_cols || height > max_rows {
            let width_ratio = max_cols as f32 / width as f32;
            let height_ratio = max_rows as f32 / height as f32;
            let scale_factor = width_ratio.min(height_ratio);
            window.0 = (width as f32 * scale_factor) as u16;
            window.1 = (height as f32 * scale_factor) as u16;
        }
    
        let mut stdout = stdout();
        execute!(stdout, terminal::EnterAlternateScreen, Hide).map_err(|e| {
            eprintln!("Erro ao configurar terminal alternativo: {:?}", e);
            e
        })?;

        Ok(window)
    }
}

// Implementação de "destrutor" com o trait Drop
impl Drop for World {
    fn drop(&mut self) {
        let mut stdout = stdout();
        // Mostra o cursor e restaura o terminal ao estado normal
        execute!(stdout, Show).expect("Erro ao mostrar o cursor");
        terminal::disable_raw_mode().expect("Erro ao desativar o modo raw");
        execute!(stdout, terminal::LeaveAlternateScreen).expect("Erro ao sair da tela alternativa");
    }
}