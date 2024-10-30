// entity.rs

// ----------------------------------------------------------------------------------------------------------------
// Entities are unique identifiers (often just integers or references) that represent individual objects
// or actors within the ECS framework. They do not hold any data or logic themselves; instead,
// they serve as a collection point for components that define their properties and behaviors.
// For instance, an entity could represent a player character, an enemy, or an object in the environment.
// ----------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
