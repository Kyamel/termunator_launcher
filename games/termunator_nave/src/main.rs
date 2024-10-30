
use termunator::utils::{clear_area, custom_print};
use termunator::utils;
use termunator::KeyCode;
use termunator::World;
use termunator::Components::*;
use termunator::Systems::*;
use utils::crossterm_interactive;


fn main() {
    termunator::hello();
    println!("Hello Nave");

    //crossterm_interactive();

    let mut world = World::World::new();

    // Event Handling
    let event_system = HandleEventsSystem::new();
    world.add_system("handle_events", event_system);

    let key = KeyState::new();
    let player_entity = world.create_entity();
    world.add_component(player_entity, key);

    // Draw

    let draw_system = DrawSystem::new();
    world.add_system("draw", draw_system);

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

    let player_system = PlayerSystem::new();
    let mut game_state = GameState::new();

    world.add_component(player_entity, body);
    world.add_component(player_entity, pos);
    world.add_component(player_entity, vel);

    world.add_system("player", player_system);


    // Movement
    //let movement_system = MovementSystem::new();
    //world.add_system("movement", movement_system);

    let window = termunator::World::World::init(16*2, 9).unwrap();
    game_state.window_size = window;

    world.add_component(player_entity, game_state);

    let target_fps = 60;

    // Game loop
    'game_loop: loop {

        clear_area(window.0, window.1);

        //let mut stdout = std::io::stdout();
        //execute!(stdout, Clear(ClearType::All)).expect("Erro ao limpar a tela");

        let keys = world.get_component::<KeyState>(player_entity).unwrap();
        if keys.is_key_pressed(KeyCode::Char('q')) {
            break 'game_loop;
        }

       world.update(60);
    }


}