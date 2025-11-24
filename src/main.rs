use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io, panic,
    time::{Duration, Instant},
};

mod game;
mod highscore;
mod ui;

use game::{Game, GameState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic_hook();

    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal);

    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut game))?;

        let timeout = game.tick_rate().saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match game.state {
                        GameState::MainMenu => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                if game.menu_selection == 0 {
                                    game.start_new_game();
                                } else if game.menu_selection == 1 {
                                    game.state = GameState::HighScores;
                                } else if game.menu_selection == 2 {
                                    return Ok(());
                                }
                            }
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                                if game.menu_selection > 0 {
                                    game.menu_selection -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                                if game.menu_selection < 2 {
                                    game.menu_selection += 1;
                                }
                            }
                            _ => {}
                        },
                        GameState::HighScores => match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Enter
                            | KeyCode::Char(' ') => {
                                game.state = GameState::MainMenu;
                            }
                            _ => {}
                        },
                        GameState::Playing => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('p') => game.state = GameState::Paused,
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                                game.change_direction(game::Direction::Up)
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                                game.change_direction(game::Direction::Down)
                            }
                            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
                                game.change_direction(game::Direction::Left)
                            }
                            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                                game.change_direction(game::Direction::Right)
                            }
                            _ => {}
                        },
                        GameState::Paused => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                game.state = GameState::MainMenu;
                            }
                            KeyCode::Char('p') | KeyCode::Enter | KeyCode::Char(' ') => {
                                game.state = GameState::Playing;
                            }
                            _ => {}
                        },
                        GameState::GameOver => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                game.state = GameState::MainMenu;
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                if game.game_over_selection == 0 {
                                    game.start_new_game();
                                } else {
                                    game.state = GameState::MainMenu;
                                }
                            }
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                                game.game_over_selection = 0;
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                                game.game_over_selection = 1;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        if last_tick.elapsed() >= game.tick_rate() {
            if game.state == GameState::Playing {
                game.update();
            }
            last_tick = Instant::now();
        }
    }
}
