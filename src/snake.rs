use std::collections::{ VecDeque, HashSet };
use crate::cell::Cell;
use crate::direction::Direction;

pub struct Snake {
    pub head: Cell,
    body: VecDeque<Direction>
}

impl Snake {
    pub fn new(head: Cell) -> Snake {
        Snake {
            head,
            body: VecDeque::new(),
        }
    }

    pub fn size(&self) -> u32 {
        self.body.len() as u32
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        self.head = self.head.translate_in_direction(direction);
        if self.body.len() > 0 {
            self.body.pop_back();
            self.body.push_front(direction.opposite());
        }
    }

    pub fn extend_body(&mut self, movement: Direction) {
        // let mut direction = movement;
        // if let Some(d) = self.body.get(self.body.len() - 1) {
        //     direction = *d;
        // }
        self.body.push_back(movement);
        
    }

    pub fn body_parts(&self) -> HashSet<Cell> {
        let mut body_parts = HashSet::new();
        
        let mut last_part = self.head;
        for next_part_direction in self.body.iter() {
            let next_part = last_part.translate_in_direction(*next_part_direction);
            body_parts.insert(next_part);
            last_part = next_part;
        }

        return body_parts;
    }
}
