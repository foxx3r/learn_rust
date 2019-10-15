struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Color { red: 255, green: 45, blue: 15 };
    bg.green = 70;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}
