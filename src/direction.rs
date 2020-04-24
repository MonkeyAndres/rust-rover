#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    W,
    E,
    S,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::W => Direction::N,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
        }
    }
}
