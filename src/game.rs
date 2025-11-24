use rand::Rng;
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    HighScores,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub snake: VecDeque<Position>,
    pub direction: Direction,
    pub next_direction: Direction,
    pub food: Position,
    pub score: u32,
    pub width: u16,
    pub height: u16,
    pub base_tick_rate: Duration,
    pub current_tick_rate: Duration,
    pub food_eaten: u32,
    pub menu_selection: usize,
    pub game_over_selection: usize,
}

impl Game {
    pub fn new() -> Self {
        let width = 40;
        let height = 20;
        let mut game = Self {
            state: GameState::MainMenu,
            snake: VecDeque::new(),
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position { x: 0, y: 0 },
            score: 0,
            width,
            height,
            base_tick_rate: Duration::from_millis(120),
            current_tick_rate: Duration::from_millis(120),
            food_eaten: 0,
            menu_selection: 0,
            game_over_selection: 0,
        };
        game.init_snake();
        game.spawn_food();
        game
    }

    fn init_snake(&mut self) {
        self.snake.clear();
        let start_x = self.width / 2;
        let start_y = self.height / 2;

        self.snake.push_back(Position {
            x: start_x,
            y: start_y,
        });
        self.snake.push_back(Position {
            x: start_x - 1,
            y: start_y,
        });
        self.snake.push_back(Position {
            x: start_x - 2,
            y: start_y,
        });
    }

    pub fn start_new_game(&mut self) {
        self.score = 0;
        self.food_eaten = 0;
        self.direction = Direction::Right;
        self.next_direction = Direction::Right;
        self.current_tick_rate = self.base_tick_rate;
        self.init_snake();
        self.spawn_food();
        self.state = GameState::Playing;
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if new_direction != self.direction.opposite() {
            self.next_direction = new_direction;
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Position {
                x: rng.gen_range(0..self.width),
                y: rng.gen_range(0..self.height),
            };

            if !self
                .snake
                .iter()
                .any(|pos| pos.x == new_food.x && pos.y == new_food.y)
            {
                self.food = new_food;
                break;
            }
        }
    }

    pub fn update(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        self.direction = self.next_direction;

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: if head.y == 0 {
                    self.height - 1
                } else {
                    head.y - 1
                },
            },
            Direction::Down => Position {
                x: head.x,
                y: (head.y + 1) % self.height,
            },
            Direction::Left => Position {
                x: if head.x == 0 {
                    self.width - 1
                } else {
                    head.x - 1
                },
                y: head.y,
            },
            Direction::Right => Position {
                x: (head.x + 1) % self.width,
                y: head.y,
            },
        };

        if self
            .snake
            .iter()
            .any(|pos| pos.x == new_head.x && pos.y == new_head.y)
        {
            self.game_over();
            return;
        }

        self.snake.push_front(new_head);

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 10;
            self.food_eaten += 1;

            if self.food_eaten % 8 == 0 {
                let reduction = Duration::from_millis(5);
                if self.current_tick_rate > Duration::from_millis(40) {
                    self.current_tick_rate = self.current_tick_rate.saturating_sub(reduction);
                }
            }

            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn game_over(&mut self) {
        use crate::highscore;

        if self.score > 0 {
            let _ = highscore::save_score(self.score, self.snake.len() as u32);
        }

        self.state = GameState::GameOver;
        self.game_over_selection = 0;
    }

    pub fn tick_rate(&self) -> Duration {
        match self.state {
            GameState::Playing => self.current_tick_rate,
            _ => Duration::from_millis(50),
        }
    }

    pub fn snake_length(&self) -> usize {
        self.snake.len()
    }

    pub fn current_speed(&self) -> f32 {
        let base_ms = self.base_tick_rate.as_millis() as f32;
        let current_ms = self.current_tick_rate.as_millis() as f32;
        base_ms / current_ms
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
