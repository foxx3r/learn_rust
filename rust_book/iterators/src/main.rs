#[derive(Debug)]

struct Shoe {
    size: u32,
    style: String
}

fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("boot") },
        Shoe { size: 10, style: String::from("allstar") },
        Shoe { size: 11, style: String::from("sandal") }
    ];

    println!("Filtered shoes:\n{:#?}", shoes_in_my_size(shoes, 10));
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    return shoes.into_iter()
        .filter(|vector| vector.size == shoe_size)
        .collect();
}
