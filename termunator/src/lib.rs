
// lib.rs

pub mod utils;
pub mod Components;
pub mod Entity;
pub mod Systems;
pub mod World;
pub use crossterm::event::KeyCode;
pub use crossterm::{execute,queue};
pub use crossterm::terminal::{Clear, ClearType};



use utils::*;

pub fn hello() {
    crossterm_hello();
    println!("Hello Engine");
}
