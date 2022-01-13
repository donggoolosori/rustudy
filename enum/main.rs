enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Up!"),
        Direction::Down => println!("Down!"),
        Direction::Left => println!("Left!"),
        Direction::Right => println!("Right!"),
    }
}
