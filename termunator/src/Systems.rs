// systems.rs

// --------------------------------------------------------------------------------------------------------------------------------
// Systems contain the logic and behavior of the ECS architecture.
// Each system operates on entities that possess a specific set of components.
// For example, a Movement System would iterate over entities with Position and Velocity components,
// updating positions based on velocity values. Systems are designed to be modular and operate independently,
//  allowing for high flexibility and parallelization.
// -------------------------------------------------------------------------------------------------------------------------------

use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::Event;


use crate::draw;
use crate::Entity::*;
use crate::Components::*;
use std::collections::HashMap;
use std::any::TypeId;

use std::time::Duration;

pub trait ISystem {
    fn update(&mut self, components: &mut HashMap<TypeId, HashMap<Entity, Box<dyn IComponent>>>);
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn new() -> Self {
        Self
    }
    
}

impl ISystem for MovementSystem {
    fn update(
        &mut self,
        components: &mut HashMap<TypeId, HashMap<Entity, Box<dyn IComponent>>>,
    ) {
        // Extrai posições e velocidades em vetores temporários
        let mut positions = components
            .remove(&TypeId::of::<Position>())
            .unwrap_or_default();

        let mut velocities = components
            .remove(&TypeId::of::<Velocity>())
            .unwrap_or_default();

         // Itera sobre todas as entidades com posições e velocidades, atualizando as posições
         for (entity, pos) in positions.iter_mut() {
            if let Some(vel) = velocities.get_mut(entity) {
                if let Some(pos) = pos.as_any_mut().downcast_mut::<Position>() {
                    if let Some(vel) = vel.as_any_mut().downcast_mut::<Velocity>() {
                        pos.x += vel.vx;
                        pos.y += vel.vy;
                    }
                }
            }
        }

        // Reinsere os componentes atualizados no HashMap
        components.insert(TypeId::of::<Position>(), positions);
        components.insert(TypeId::of::<Velocity>(), velocities);
    }
}

pub struct HandleEventsSystem;

impl HandleEventsSystem {
    pub fn new() -> Self {
        Self
    }

}

impl ISystem for HandleEventsSystem {
    fn update(
        &mut self,
        components: &mut HashMap<TypeId,
        HashMap<Entity, Box<dyn IComponent>>>
    )  {
        // Remove e obtém o componente KeyState
        let keys_component_map = components
            .get_mut(&TypeId::of::<KeyState>())
            .expect("KeyState component not found");

        // Seleciona uma entidade arbitrária que possui KeyState (pode ser ajustado se houver várias)
        let key_state_component = keys_component_map
            .values_mut()
            .next()
            .and_then(|component| component.as_any_mut().downcast_mut::<KeyState>())
            .expect("Failed to downcast to KeyState");

        // Libera teclas que não estão mais pressionadas
        let pressed_keys: Vec<KeyCode> = key_state_component.keys.keys().cloned().collect();
        for key in pressed_keys {
            key_state_component.update_key_state(key, false);
        }

        // Libera teclas que não estão mais pressionadas
        let pressed_keys: Vec<KeyCode> = key_state_component.keys.keys().cloned().collect();
        for key in pressed_keys {
            key_state_component.update_key_state(key, false);
        }

        // Processa eventos e atualiza estados
        if event::poll(Duration::from_millis(0)).expect("Erro ao verificar input") {
            if let Event::Key(key_event) = event::read().expect("Erro ao ler evento") {
                match key_event.code {
                    KeyCode::Char(letra) => {
                        key_state_component.update_key_state(KeyCode::Char(letra), true);
                    },
                    _ => {} // Ignore outras teclas
                }
            }
        }

        // Limpa eventos pendentes para evitar acúmulo de inputs
        while event::poll(Duration::from_millis(0)).unwrap() {
            event::read().unwrap(); // Lê e descarta o evento
        }
    }
}

pub struct DrawSystem;

impl DrawSystem {
    pub fn new() -> Self {
        Self
    }
}

impl ISystem for DrawSystem {
    fn update(&mut self, components: &mut HashMap<TypeId, HashMap<Entity, Box<dyn IComponent>>>) {
        // Extrai bodies e posições em vetores temporários
        let mut bodies = components
            .remove(&TypeId::of::<Body>())
            .unwrap_or_default();

        let mut positions = components
            .remove(&TypeId::of::<Position>())
            .unwrap_or_default();

        let mut game_states = components
            .remove(&TypeId::of::<GameState>())
            .unwrap_or_default();

        // Itera sobre todas as entidades com bodies, posições e estados de jogo
        for (entity, pos) in positions.iter_mut() {
            if let Some(body) = bodies.get_mut(entity) {
                // Usa downcast para converter o `Box<dyn IComponent>` de volta para `Body`
                if let Some(body) = body.as_any_mut().downcast_mut::<Body>() {
                    // Obtém o estado de jogo correspondente
                    if let Some(state) = game_states.get_mut(entity) {
                        if let Some(state) = state.as_any_mut().downcast_mut::<GameState>() {

                            // Desenha
                            draw(body, pos.as_any().downcast_ref::<Position>().unwrap(), &state.window_size);

                        }
                    }
                }
            }
        }

        // Reinsere os componentes atualizados no HashMap
        components.insert(TypeId::of::<Body>(), bodies);
        components.insert(TypeId::of::<Position>(), positions);
        components.insert(TypeId::of::<GameState>(), game_states);

    }
}

pub struct PlayerSystem;

impl PlayerSystem {
    pub fn new() -> Self {
        Self
    }
}

// Definição do sistema de jogador
impl ISystem for PlayerSystem {
    fn update(&mut self, components: &mut HashMap<TypeId, HashMap<Entity, Box<dyn IComponent>>>) {
        // Remove `bodies`, `positions` e `velocities` do HashMap temporariamente
        let mut positions = components.remove(&TypeId::of::<Position>()).unwrap_or_default();
        let velocities = components.remove(&TypeId::of::<Velocity>()).unwrap_or_default();

        // Obtém e processa `KeyState`
        let key_state_component = components.get_mut(&TypeId::of::<KeyState>())
            .expect("KeyState component not found")
            .values_mut()
            .next()
            .expect("No KeyState component found");

        // Downcast para KeyState
        let key_state = key_state_component.as_any_mut()
            .downcast_mut::<KeyState>()
            .expect("Failed to downcast to KeyState");

        // Itera sobre cada entidade que possui Velocity e Position
        for (entity, velocity) in velocities.iter() {
            if let Some(position) = positions.get_mut(entity)
                .and_then(|component| component.as_any_mut().downcast_mut::<Position>()) 
            {
                // Downcast de `velocity` para acessar `vx` e `vy`
                let velocity = velocity.as_any()
                    .downcast_ref::<Velocity>()
                    .expect("Failed to downcast to Velocity");

                // Verifica as teclas pressionadas e atualiza a posição
                if key_state.is_key_pressed(KeyCode::Char('w')) {
                    position.y -= velocity.vy; // Move para cima
                }
                if key_state.is_key_pressed(KeyCode::Char('s')) {
                    position.y += velocity.vy; // Move para baixo
                }
                if key_state.is_key_pressed(KeyCode::Char('a')) {
                    position.x -= velocity.vx; // Move para a esquerda
                }
                if key_state.is_key_pressed(KeyCode::Char('d')) {
                    position.x += velocity.vx; // Move para a direita
                }
            }
        }

        // Reinsere os componentes atualizados no HashMap
        components.insert(TypeId::of::<Position>(), positions);
        components.insert(TypeId::of::<Velocity>(), velocities);
    }
}