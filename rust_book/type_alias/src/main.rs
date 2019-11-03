fn main() {
    type Item = String;

    fn person(name: Item) -> Item {
        return String::from(format!("Hello, {}", name));
    }

    let james = person(String::from("James"));

    println!("{}", james);
}
