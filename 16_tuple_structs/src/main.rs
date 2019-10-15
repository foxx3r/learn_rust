struct Color(u8, u8, u8);

fn main() {
    let bg = Color(255, 0, 0);

    println!("The color is {}, {}, {}", bg.0, bg.1, bg.2);
}
