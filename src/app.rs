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
    pub record: u32,
    status: AppStatus,
    game: Game,
    pending_movement: Option<Direction>,
    cycle_elapsed_time: f64
}

const CYCLE_TIME: f64 = 0.1;
const DIMENSIONS: Dimensions = Dimensions { width: 17, height: 17 };

impl App {
    pub fn new() -> App {
        App {
            record: 0,
            status: AppStatus::Playing,
            game: Game::new(DIMENSIONS),
            pending_movement: None,
            cycle_elapsed_time: 0.0
        }
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        clear([1.0; 4], graphics);

        // render snake
        rectangle(
            [0.0, 1.0, 0.0, 1.0], // green
            [self.game.snake.head.x as f64 * 30.0, self.game.snake.head.y as f64 * 30.0, 30.0, 30.0],
            context.transform,
            graphics
        );
        for cell in self.game.snake.body_parts() {
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [cell.x as f64 * 30.0, cell.y as f64 * 30.0, 30.0, 30.0],
                context.transform,
                graphics
            );
        }

        // render apple
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            [self.game.apple.x as f64 * 30.0, self.game.apple.y as f64 * 30.0, 30.0, 30.0],
            context.transform,
            graphics
        );
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
        self.game = Game::new(DIMENSIONS);
        self.pending_movement = None;
        self.cycle_elapsed_time = 0.0;
    }
}