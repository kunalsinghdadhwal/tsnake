use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::game::{Game, GameState};
use crate::highscore;

pub fn draw(f: &mut Frame, game: &mut Game) {
    match game.state {
        GameState::MainMenu => draw_main_menu(f, game),
        GameState::HighScores => draw_high_scores(f),
        GameState::Playing => draw_game(f, game),
        GameState::Paused => {
            draw_game(f, game);
            draw_pause_overlay(f);
        }
        GameState::GameOver => {
            draw_game(f, game);
            draw_game_over(f, game);
        }
    }
}

fn draw_main_menu(f: &mut Frame, game: &Game) {
    let area = f.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .style(Style::default().bg(Color::Black))
        .title(" TSNAKE ")
        .title_alignment(Alignment::Center);

    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    let title = vec![
        Line::from(""),
        Line::from(Span::styled(
            "███████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "██╔════╝████╗  ██║██╔══██╗██║ ██╔╝██╔════╝",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "███████╗██╔██╗ ██║███████║█████╔╝ █████╗  ",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "╚════██║██║╚██╗██║██╔══██║██╔═██╗ ██╔══╝  ",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "███████║██║ ╚████║██║  ██║██║  ██╗███████╗",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
    ];

    let title_widget = Paragraph::new(title);
    f.render_widget(title_widget, chunks[0]);

    let menu_items = ["Play", "High Scores", "Quit"];
    let mut menu_lines = vec![Line::from("")];

    for (i, item) in menu_items.iter().enumerate() {
        let style = if i == game.menu_selection {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightCyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if i == game.menu_selection { "> " } else { "  " };
        menu_lines.push(
            Line::from(Span::styled(format!("{}{}", prefix, item), style))
                .alignment(Alignment::Center),
        );
        menu_lines.push(Line::from(""));
    }

    menu_lines.push(Line::from(""));
    menu_lines.push(
        Line::from(Span::styled(
            "Use Arrow Keys / WASD / HJKL to move",
            Style::default().fg(Color::DarkGray),
        ))
        .alignment(Alignment::Center),
    );
    menu_lines.push(
        Line::from(Span::styled(
            "Press P to pause • Q/Esc to quit",
            Style::default().fg(Color::DarkGray),
        ))
        .alignment(Alignment::Center),
    );

    let menu_widget = Paragraph::new(menu_lines);
    f.render_widget(menu_widget, chunks[1]);
}

fn draw_high_scores(f: &mut Frame) {
    let area = f.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .style(Style::default().bg(Color::Black))
        .title(" HIGH SCORES ")
        .title_alignment(Alignment::Center);

    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([Constraint::Min(0)])
        .split(area);

    let scores = highscore::load_scores().unwrap_or_default();

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "TOP 10 SCORES",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  RANK  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "SCORE  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "LENGTH  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "DATE",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "─".repeat(60),
            Style::default().fg(Color::DarkGray),
        ))
        .alignment(Alignment::Center),
    ];

    if scores.is_empty() {
        lines.push(Line::from(""));
        lines.push(
            Line::from(Span::styled(
                "No high scores yet. Play to set one!",
                Style::default().fg(Color::DarkGray),
            ))
            .alignment(Alignment::Center),
        );
    } else {
        for (i, score) in scores.iter().enumerate().take(10) {
            let rank_color = match i {
                0 => Color::Yellow,
                1 => Color::LightCyan,
                2 => Color::LightRed,
                _ => Color::White,
            };

            let date_str = score.timestamp.format("%Y-%m-%d %H:%M").to_string();

            lines.push(
                Line::from(vec![
                    Span::styled(
                        format!("   {:2}.   ", i + 1),
                        Style::default().fg(rank_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:5}  ", score.score),
                        Style::default().fg(Color::Green),
                    ),
                    Span::styled(
                        format!("{:6}  ", score.length),
                        Style::default().fg(Color::Magenta),
                    ),
                    Span::styled(date_str, Style::default().fg(Color::DarkGray)),
                ])
                .alignment(Alignment::Center),
            );
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "Press any key to return to menu",
            Style::default().fg(Color::DarkGray),
        ))
        .alignment(Alignment::Center),
    );

    let widget = Paragraph::new(lines);
    f.render_widget(widget, chunks[0]);
}

fn draw_game(f: &mut Frame, game: &Game) {
    let area = f.area();

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    draw_header(f, game, vertical_chunks[0]);

    let game_width = game.width * 2 + 2;
    let game_height = game.height + 2;

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(game_width),
            Constraint::Min(0),
        ])
        .split(vertical_chunks[1]);

    let vertical_game_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(game_height),
            Constraint::Min(0),
        ])
        .split(horizontal_chunks[1]);

    let game_area = vertical_game_chunks[1];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Rgb(20, 20, 30)))
        .title(" GAME ")
        .title_alignment(Alignment::Center);

    f.render_widget(block, game_area);

    let inner_area = Rect {
        x: game_area.x + 1,
        y: game_area.y + 1,
        width: game_area.width.saturating_sub(2),
        height: game_area.height.saturating_sub(2),
    };

    let mut grid = vec![vec![' '; game.width as usize]; game.height as usize];

    for (i, segment) in game.snake.iter().enumerate() {
        if segment.y < game.height && segment.x < game.width {
            grid[segment.y as usize][segment.x as usize] = if i == 0 { 'H' } else { 'B' };
        }
    }

    if game.food.y < game.height && game.food.x < game.width {
        grid[game.food.y as usize][game.food.x as usize] = 'F';
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let cell_x = inner_area.x + (x as u16 * 2);
            let cell_y = inner_area.y + y as u16;

            if cell_x + 1 < inner_area.x + inner_area.width
                && cell_y < inner_area.y + inner_area.height
            {
                let (symbol, style) = match cell {
                    'H' => (
                        "⬤ ",
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    ),
                    'B' => (
                        "■ ",
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    ),
                    'F' => (
                        "❤ ",
                        Style::default()
                            .fg(Color::LightRed)
                            .add_modifier(Modifier::BOLD),
                    ),
                    _ => ("  ", Style::default()),
                };

                let cell_widget = Paragraph::new(symbol).style(style);
                let cell_area = Rect {
                    x: cell_x,
                    y: cell_y,
                    width: 2,
                    height: 1,
                };
                f.render_widget(cell_widget, cell_area);
            }
        }
    }
}

fn draw_header(f: &mut Frame, game: &Game, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightBlue))
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: 1,
    };

    let score_text = format!(" Score: {} ", game.score);
    let length_text = format!(" Length: {} ", game.snake_length());
    let speed_text = format!(" Speed: {:.1}x ", game.current_speed());

    let header_line = Line::from(vec![
        Span::styled(
            score_text,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            length_text,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            speed_text,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ])
    .alignment(Alignment::Center);

    let header_widget = Paragraph::new(header_line);
    f.render_widget(header_widget, inner);
}

fn draw_pause_overlay(f: &mut Frame) {
    let area = f.area();

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(7),
            Constraint::Percentage(40),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(vertical_chunks[1]);

    let popup_area = horizontal_chunks[1];

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, popup_area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "PAUSED",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(
            "Press P or Space to resume",
            Style::default().fg(Color::White),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "Press Q or Esc to quit",
            Style::default().fg(Color::DarkGray),
        ))
        .alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, popup_area);
}

fn draw_game_over(f: &mut Frame, game: &Game) {
    let area = f.area();

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(12),
            Constraint::Percentage(30),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(vertical_chunks[1]);

    let popup_area = horizontal_chunks[1];

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, popup_area);

    let mut text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "GAME OVER",
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled("Final Score: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{}", game.score),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .alignment(Alignment::Center),
        Line::from(vec![
            Span::styled("Length: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{}", game.snake_length()),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .alignment(Alignment::Center),
        Line::from(""),
    ];

    let play_style = if game.game_over_selection == 0 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightCyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let quit_style = if game.game_over_selection == 1 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightCyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    text.push(
        Line::from(Span::styled(
            if game.game_over_selection == 0 {
                "> Play Again"
            } else {
                "  Play Again"
            },
            play_style,
        ))
        .alignment(Alignment::Center),
    );
    text.push(
        Line::from(Span::styled(
            if game.game_over_selection == 1 {
                "> Main Menu"
            } else {
                "  Main Menu"
            },
            quit_style,
        ))
        .alignment(Alignment::Center),
    );

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, popup_area);
}
