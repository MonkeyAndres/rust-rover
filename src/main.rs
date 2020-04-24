mod direction;
mod grid_point;

use direction::Direction;
use grid_point::GridPoint;

#[derive(Debug, Clone, Copy)]
enum Command {
    L,
    R,
    F,
    B,
}

const STARTING_POINT: GridPoint = GridPoint::new(0, 0);

const INITIAL_DIRECTION: Direction = Direction::E;

fn rover(commands: &[Command]) -> (GridPoint, Direction) {
    let mut position = STARTING_POINT;
    let mut direction = INITIAL_DIRECTION;

    for command in commands {
        match command {
            Command::L => direction = direction.turn_left(),
            Command::R => direction = direction.turn_right(),
            Command::F => position = position.move_forward(direction),
            Command::B => position = position.move_backwards(direction),
        }
    }

    (position, direction)
}

fn main() {
    let example_commands: Vec<Command> = vec![
        Command::F,
        Command::B,
        Command::R,
        Command::L,
        Command::F,
        Command::F,
        Command::F,
    ];

    let result = rover(&example_commands[..3]);
    let result2 = rover(&example_commands[3..]);
    let result3 = rover(&example_commands);

    println!("{:?} - {:?}", result, result2)
}
