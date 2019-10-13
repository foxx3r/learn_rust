fn main() {
    let animals = vec!["Rabbit", "Dog", "Shark"];

    for animal in animals.iter() {
        println!("The animal is {}", animal);
    }

    for counter in 1..11 {
        println!("{}", counter);
    }
}
