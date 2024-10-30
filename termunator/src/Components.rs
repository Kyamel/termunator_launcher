// components.rs

// --------------------------------------------------------------------------------------------------------------------------------
// Components are data containers that store individual attributes or states related to entities.
// Each component represents a single aspect of an entity, such as its position, velocity, health, or appearance.
// Components are purely data-oriented; they contain no behavior or logic.
// In ECS, entities can have any combination of components, which defines their
// specific characteristics and makes them flexible and easily configurable.
// -------------------------------------------------------------------------------------------------------------------------------

use std::{any::Any, collections::HashMap};

use crossterm::event::KeyCode;

pub trait IComponent: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl IComponent for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self { vx, vy }
    }
}

impl IComponent for Velocity {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Body {
    pub mat: Vec<Vec<char>>,
}

impl Body {
    pub fn new(mat: Vec<Vec<char>>) -> Self {
        Self { mat }
    }
}

impl IComponent for Body {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Body {
    pub fn size(&self) -> (i32, i32) {
        let body_height = self.mat.len() as i32;
        let body_width = self.mat.get(0).map_or(0, |row| row.len() as i32);

        (body_width, body_height)
    }
}

#[derive(Debug, Clone)]
pub struct KeyState {
    pub keys: HashMap<KeyCode, bool>, // Armazena se cada tecla está pressionada ou não
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    // Atualiza o estado de uma tecla (pressionada ou não)
    pub fn update_key_state(&mut self, key: KeyCode, pressed: bool) {
        self.keys.insert(key, pressed);
    }

    // Retorna se uma tecla está pressionada
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }
}

impl IComponent for KeyState {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub running: bool,
    pub paused: bool,
    pub delta_time: f32,
    pub time: f32,
    pub window_size: (u16, u16),
}

impl GameState {
    pub fn new() -> Self {
        Self {
            running: true,
            paused: false,
            delta_time: 60.0,
            time: 0.0,
            window_size: (32, 9),
        }
    }
}

impl IComponent for GameState {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}