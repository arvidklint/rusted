use std::convert::TryInto;

use crate::math::clamp;
use crate::utils::Position;

pub struct Cursor {
    pub position: Position,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn row(&self) -> usize {
        if let Ok(x) = self.position.x.try_into() {
            x
        } else {
            0
        }
    }

    pub fn col(&self) -> usize {
        if let Ok(y) = self.position.y.try_into() {
            y
        } else {
            0
        }
    }

    pub fn set_position(&mut self, x: isize, y: isize) {
        self.position.x = clamp(x, 0, isize::MAX);

        self.position.y = clamp(y, 0, isize::MAX);
    }
}
