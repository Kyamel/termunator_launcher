use crossterm::{cursor::{Hide, MoveTo, Show}, execute, style::Print, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};

pub fn crossterm_hello() {
    execute!(stdout(), Print("Hello Crossterm\n")).expect("Erro ao imprimir no console");
}

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    queue,
    terminal::{self},
};

use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::Components::*;

pub fn init(width: u16, height: u16) -> (u16, u16) {
    // Configuração inicial
    terminal::enable_raw_mode().expect("Erro ao ativar o modo raw");

    // Obtém o tamanho atual do terminal
    let (max_cols, max_rows) = terminal::size().expect("Erro ao obter tamanho do console");
    let mut window = (width, height);

    // Verifica se o tamanho informado está dentro dos limites
    if width > max_cols || height > max_rows {
        // Calcula a proporção de ajuste
        let width_ratio = max_cols as f32 / width as f32;
        let height_ratio = max_rows as f32 / height as f32;

        // Usa o menor fator para reduzir ambas as dimensões proporcionalmente
        let scale_factor = width_ratio.min(height_ratio);
        window.0 = (width as f32 * scale_factor) as u16;
        window.1 = (height as f32 * scale_factor) as u16;
    }

    // Configuração de terminal alternativo e ocultação do cursor
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, Hide).expect("Erro ao configurar terminal alternativo");

    window
}

fn exit() {
    let mut stdout = stdout();
    // Restaura o cursor
    execute!(stdout, Show).expect("Erro ao mostrar o cursor");
    // Restaura o estado do terminal
    terminal::disable_raw_mode().expect("Erro ao desativar o modo raw");
    execute!(stdout, terminal::LeaveAlternateScreen).expect("Erro ao sair da tela alternativa");
}

pub fn crossterm_interactive() {

    let mut stdout = stdout();
    let window = init(16*2, 9);

    // Variável para FPS
    let target_fps = 60;
    let frame_duration = Duration::from_millis(1000 / target_fps);
    let mut state = 0;

    let pos = Position { x: 14.0, y: 5.0 };
    let body = Body {
        mat: vec![vec!['O'; 3]; 2],
    };

    // Game loop
    'game_loop: loop {
        let start_time = Instant::now();

        // Input - verifica por 'q' para sair
        if event::poll(Duration::from_millis(0)).expect("Erro ao verificar input") {
            if let Event::Key(key_event) = event::read().expect("Erro ao ler evento") {
                if key_event.code == KeyCode::Char('q') {
                    break 'game_loop;
                }
                if key_event.code == KeyCode::Char('a') {
                    state+=1;
                }
            }
        }

        // Limpeza e atualização - aqui você adiciona a lógica do jogo e o redesenho
        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            Print(format!("Game rodando a {}fps no terminal!\n", target_fps)),
            Print(format!("Window of size: {}:{}\n", window.0, window.1)),
            Print("Pressione 'q' para sair.\n"),
            Print(format!("'a' pressionado: {} vezes", state)),
        ).expect("Erro ao enfileirar comandos");

        draw_screen_border(&window);
        draw(&body, &pos, &window);

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

    exit();
}


pub fn draw(body: &Body, pos: &Position, window: &(u16, u16)) {
    let (win_width, win_height) = window;
    let mut stdout = stdout();

    // Converte `Position` para inteiros, arredondando
    let start_x = pos.x as i32;
    let start_y = pos.y as i32;

    // Dimensões do `body`
    let (body_width, body_height) = body.size();

    // Itera sobre as células visíveis do `body`
    for row in 0..body_height {
        let draw_y = start_y + row;
        if draw_y < 0 || draw_y >= *win_height as i32 {
            continue; // Pule linhas fora da janela
        }

        for col in 0..body_width {
            let draw_x = start_x + col;
            if draw_x < 0 || draw_x >= *win_width as i32 {
                continue; // Pule colunas fora da janela
            }

            // Posição e valor da célula a ser desenhada
            let cell_value = body.mat[row as usize][col as usize];
            execute!(
                stdout,
                MoveTo(draw_x as u16, draw_y as u16),
                Print(format!("{}", cell_value))
            ).expect("Erro ao desenhar no terminal");
            stdout.flush().expect("Erro ao dar flush na saída");
        }
    }
}

pub fn draw_screen_border(window: &(u16, u16)) {
    let (win_width, win_height) = window;
    let mut stdout = stdout();

    // Desenha as linhas superior e inferior
    for x in 0..*win_width {
        // Linha superior
        execute!(
            stdout,
            MoveTo(x, 0),
            Print("*")
        ).expect("Erro ao desenhar no terminal");

        // Linha inferior
        execute!(
            stdout,
            MoveTo(x, win_height - 1),
            Print("*")
        ).expect("Erro ao desenhar no terminal");
    }

    // Desenha as colunas esquerda e direita
    for y in 0..*win_height {
        // Coluna esquerda
        execute!(
            stdout,
            MoveTo(0, y),
            Print("*")
        ).expect("Erro ao desenhar no terminal");

        // Coluna direita
        execute!(
            stdout,
            MoveTo(win_width - 1, y),
            Print("*")
        ).expect("Erro ao desenhar no terminal");
    }
}

pub fn clear_area(width: u16, height: u16) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::CurrentLine)).expect("Erro ao mover o cursor");

    for _ in 0..height {
        print!("{:width$}", "", width = width as usize); // Imprime espaços em branco para limpar a linha
        print!("\n"); // Move para a próxima linha
    }
    stdout.flush().expect("Erro ao atualizar o terminal");
}

pub fn custom_print(message: &str) {
    let mut stdout = stdout();
    stdout.flush().expect("Erro ao dar flush na saída");
    MoveTo(0, 0);
    execute!(stdout, Print(message)).expect("Erro ao imprimir");
    stdout.flush().expect("Erro ao dar flush na saída");
}
