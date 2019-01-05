use crate::direction::Direction;
use crate::dimensions::Dimensions;
use rand::Rng;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Cell {
    pub x: i8,
    pub y: i8
}

impl Cell {
    pub fn new(x: i8, y: i8) -> Cell {
        Cell { x, y }
    }

    pub fn random(dimensions: Dimensions) -> Cell {
        Cell::new(
            rand::thread_rng().gen_range(0, dimensions.width),
            rand::thread_rng().gen_range(0, dimensions.height),
        )
    }

    pub fn translate_in_direction(&self, direction: Direction) -> Cell {
        match direction {
            Direction::Up => Cell { y: self.y - 1, x: self.x },
            Direction::Down => Cell { y: self.y + 1, x: self.x },
            Direction::Left => Cell { x: self.x - 1, y: self.y },
            Direction::Right => Cell { x: self.x + 1, y: self.y }
        }
    }
}
