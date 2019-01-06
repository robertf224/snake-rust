use piston_window::*;
use std::cmp;
use crate::game::{ Game, GameStatus };
use crate::direction::Direction;
use crate::dimensions::Dimensions;

enum AppStatus {
    Playing,
    Paused
}

pub struct App {
    font: Glyphs,
    pub record: u32,
    status: AppStatus,
    game: Game,
    pending_movement: Option<Direction>,
    cycle_elapsed_time: f64
}

const CYCLE_TIME: f64 = 0.1;
const GAME_DIMENSIONS: Dimensions = Dimensions { width: 17, height: 17 };
const PADDING: f64 = 20.0;
const FONT_SIZE: u32 = 16;

impl App {
    pub fn new(font: Glyphs) -> App {
        App {
            font,
            record: 0,
            status: AppStatus::Playing,
            game: Game::new(GAME_DIMENSIONS),
            pending_movement: None,
            cycle_elapsed_time: 0.0
        }
    }

    pub fn render(&mut self, context: Context, graphics: &mut G2d) {
        let [width, height] = context.get_view_size();
        let hud_height = (FONT_SIZE as f64) + PADDING * 2.0;
        let game_height = (height as f64) - hud_height - PADDING * 2.0;
        let game_width = (width as f64) - PADDING * 2.0;
        let game_size = cmp::min(game_width as u32, game_height as u32) as f64;
        let cell_width = game_size / (GAME_DIMENSIONS.width as f64);
        let cell_height = game_size / (GAME_DIMENSIONS.height as f64);
        let game_x = width / 2.0 - game_size / 2.0;
        let game_y = (height - hud_height) / 2.0 + hud_height - game_size / 2.0;

        clear([1.0; 4], graphics);

        // render game
        rectangle(
            [0.5, 0.5, 0.5, 0.5], // light gray
            [0.0, hud_height, width, height - hud_height],
            context.transform,
            graphics
        );

        // render board
        rectangle(
            [0.9, 0.9, 0.9, 1.0], // gray
            [game_x, game_y, game_size, game_size],
            context.transform,
            graphics
        );

        // render snake
        rectangle(
            [0.0, 1.0, 0.0, 1.0], // green
            [game_x + self.game.snake.head.x as f64 * cell_width, game_y + self.game.snake.head.y as f64 * cell_height, cell_width, cell_height],
            context.transform,
            graphics
        );
        for cell in self.game.snake.body_parts() {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [game_x + cell.x as f64 * cell_width, game_y + cell.y as f64 * cell_height, cell_width, cell_height],
                context.transform,
                graphics
            );
        }

        // render apple
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            [game_x + self.game.apple.x as f64 * cell_width, game_y + self.game.apple.y as f64 * cell_height, cell_width, cell_height],
            context.transform,
            graphics
        );

        // render HUD
        text(
            [1.0, 0.0, 0.0, 1.0], // red
            FONT_SIZE,
            &format!("Score: {} | Record: {}", self.game.score(), self.record),
            &mut self.font,
            context.transform.trans(PADDING, (FONT_SIZE as f64) + PADDING),
            graphics
        ).unwrap();
    }

    pub fn update(&mut self, dt: f64) {
        if let AppStatus::Paused = self.status {
            return;
        }
        if let GameStatus::Over = self.game.status { 
            return;
        }

        let cycle_elapsed_time = self.cycle_elapsed_time + dt;
        if cycle_elapsed_time > CYCLE_TIME {
            self.game.update(self.pending_movement);
            self.cycle_elapsed_time = 0.0;
            self.pending_movement = None;
        } else {
            self.cycle_elapsed_time += dt;
        }

        if let GameStatus::Over = self.game.status {
            self.record = cmp::max(self.game.score(), self.record);
        }
    }

    pub fn press(&mut self, button: Button) {
        if let Button::Keyboard(key) = button {
            self.pending_movement = match key {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None
            };

            if let Key::Space = key {
                self.status = match self.status {
                    AppStatus::Playing => AppStatus::Paused,
                    AppStatus::Paused => AppStatus::Playing
                };
            }

            if let Key::R = key {
                self.restart();
            }
        }
    }

    fn restart(&mut self) {
        self.status = AppStatus::Playing;
        self.game = Game::new(GAME_DIMENSIONS);
        self.pending_movement = None;
        self.cycle_elapsed_time = 0.0;
    }
}