use crate::cell::Cell;
use crate::direction::Direction;
use crate::dimensions::Dimensions;
use crate::snake::Snake;

pub enum GameStatus {
    Playing,
    Over
}

pub struct Game {
    pub dimensions: Dimensions,
    pub snake: Snake,
    pub apple: Cell,
    pub status: GameStatus,
    last_movement: Direction
}

impl Game {
    pub fn new(dimensions: Dimensions) -> Game {
        Game {
            dimensions,
            snake: Snake::new(Cell::new(dimensions.width / 2, dimensions.height / 2)),
            apple: Cell::random(dimensions),
            status: GameStatus::Playing,
            last_movement: Direction::Right
        }
    }

    pub fn score(&self) -> u32 {
        self.snake.size() * 10
    }

    pub fn update(&mut self, maybe_movement: Option<Direction>) {
        if let GameStatus::Over = self.status {
            return;
        }

        let movement: Direction = match maybe_movement {
            None => self.last_movement,
            Some(movement) => {
                if movement == self.last_movement.opposite() {
                    self.last_movement
                } else {
                    movement
                }
            }
        };

        self.snake.move_in_direction(movement);
        if self.snake.head.x < 0 || self.snake.head.x >= self.dimensions.width ||
           self.snake.head.y < 0 || self.snake.head.y >= self.dimensions.height ||
           self.snake.body_parts().contains(&self.snake.head) {
            self.status = GameStatus::Over;
        } else {
            if self.snake.head == self.apple {
                self.snake.extend_body(movement);
                self.apple = Cell::random(self.dimensions);
            }
            self.last_movement = movement;
        }
    }
}