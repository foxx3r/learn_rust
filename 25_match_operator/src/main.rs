fn main() {
    let number = 4;

    match number {
        1 => println!("It is one"),
        2...6 => println!("It is between two and six"),
        7 | 8 => println!("It is seven or eight"),
        _ => println!("It is unknown")
    }
}
