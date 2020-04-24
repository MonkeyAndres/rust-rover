use crate::direction::Direction;

const GRID_DIMENSIONS: GridPoint = GridPoint::new(20, 20);

#[derive(Debug, Clone, Copy)]
pub struct GridPoint(usize, usize);

impl GridPoint {
    pub const fn new(x: usize, y: usize) -> GridPoint {
        GridPoint(x, y)
    }

    pub fn move_forward(self, direction: Direction) -> GridPoint {
        let GridPoint(mut x, mut y) = self;
        let GridPoint(max_x, max_y) = GRID_DIMENSIONS;

        match direction {
            Direction::N => {
                if y < max_y {
                    y += 1
                }
            }
            Direction::W => {
                if x > 0 {
                    x -= 1
                }
            }
            Direction::E => {
                if x < max_x {
                    x += 1
                }
            }
            Direction::S => {
                if y > 0 {
                    y -= 1
                }
            }
        }
        GridPoint(x, y)
    }

    pub fn move_backwards(self, direction: Direction) -> GridPoint {
        let GridPoint(mut x, mut y) = self;
        let GridPoint(max_x, max_y) = GRID_DIMENSIONS;

        match direction {
            Direction::N => {
                if y > 0 {
                    y -= 1
                }
            }
            Direction::W => {
                if x < max_x {
                    x += 1
                }
            }
            Direction::E => {
                if x > 0 {
                    x -= 1
                }
            }
            Direction::S => {
                if y < max_y {
                    y += 1
                }
            }
        }
        GridPoint(x, y)
    }
}
