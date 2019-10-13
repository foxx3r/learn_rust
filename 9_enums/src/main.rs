enum Direction {
    Up,
    Right,
    Left,
    Down
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Right => println!("I'm go to right"),
        Direction::Left => println!("You should go to the left, maybe you can find anything"),
        Direction::Down => println!("Oh, it's below of us")
    }
}
