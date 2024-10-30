
use termunator::utils::{clear_area, custom_print};
use termunator::KeyCode;
use termunator::World;
use termunator::Components::*;
use termunator::Systems::*;

fn main() {
    termunator::hello();
    println!("Hello Nave");

    // Initialize World
    let mut world = World::World::new();

    // Add systems
    let event_system = HandleEventsSystem::new();
    let draw_system = DrawSystem::new();
    let player_system = PlayerSystem::new();

    world.add_system("handle_events", event_system);
    world.add_system("draw", draw_system);
    world.add_system("player", player_system);

    // Initialize player
    let player_entity = world.create_entity();

    // Add components to player
    let key = KeyState::new();
    let body = Body::new(
        vec![
                vec![' ', '^', '^', ' '],
                vec!['/', '0', '0', '\\'],
                vec!['|', '=', '=', '|'],
                vec![' ', '/', '\\', ' '],
            ]
        );
    let pos = Position::new(0.0, 5.0);
    let vel = Velocity::new(1.0, 1.0);

    world.add_component(player_entity, key);
    world.add_component(player_entity, body);
    world.add_component(player_entity, pos);
    world.add_component(player_entity, vel);

    // Initialize window
    let mut game_state = GameState::new();
    let window = termunator::World::World::init(16*2, 9).unwrap();
    game_state.window_size = window;
    world.add_component(player_entity, game_state);

    'game_loop: loop {

        clear_area(window.0, window.1);

        let keys = world.get_component::<KeyState>(player_entity).unwrap();
        if keys.is_key_pressed(KeyCode::Char('q')) {
            break 'game_loop;
        }

       world.update(60);
    }

}