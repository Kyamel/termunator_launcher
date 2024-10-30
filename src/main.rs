use std::fs;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    // Lê dinamicamente os jogos na pasta `games`
    let games = get_games_list();

    // Exibe o menu com os jogos disponíveis
    if games.is_empty() {
        println!("No games found in `games`.");
        return;
    }

    loop {
        println!("\x1b[35m\nGame Launcher:\x1b[31m");
        println!("{}. exit\x1b[33m", 0);
        for (index, game) in games.iter().enumerate() {
            println!("{}. {}", index + 1, game);
        }
        println!("\x1b[32m{}. how to make a game", games.len() + 1);

        // Lê a escolha do usuário
        print!("\x1b[34mEnter your choice number: \x1b[0m");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<usize>().unwrap_or(0);

        // Verifica a opção selecionada
        if choice > 0 && choice <= games.len() {
            run_game(&games[choice - 1]);
        } else if choice == games.len() + 1 {
            how_to_use_dialog();
        } else if choice == 0 {
            println!("Exiting...");
            break; // Sai do loop e encerra o programa
        } else {
            println!("Invalid choice!"); // Loop continua e exibe o menu novamente
        }
    }
}

fn how_to_use_dialog() {
    println!("Instructions: ");

    println!("\nPress Enter to continue.");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap();
}

// Lê o conteúdo da pasta `games` e retorna uma lista com o nome de cada subpasta
fn get_games_list() -> Vec<String> {
    let mut games = Vec::new();

    if let Ok(entries) = fs::read_dir("games") {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(game_name) = entry.file_name().to_str() {
                    games.push(game_name.to_string());
                }
            }
        }
    }
    games
}

// Executa o jogo selecionado
fn run_game(game_name: &str) {
    let game_path = format!("games/{}", game_name); // Caminho completo para o jogo

    println!("Initializing game {}...", game_name);
    let status = Command::new("cargo")
        .arg("run")
        .current_dir(game_path) // Define o diretório do jogo
        .status()
        .expect("Failed to initialize game");

    if !status.success() {
        println!("Error initializing game {}", game_name);
    }
}