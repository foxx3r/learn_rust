struct Person {
    name: String,
    age: u8
}

trait Speak {
    fn presentation(&self) -> String;
}

impl Speak for Person {
    fn presentation(&self) -> String {
        return format!("Hello, I'm {} years old and my name is {}, how it's going?", self.age, self.name);
    }
}

fn main() {
    let gustavo = Person { name: String::from("Gustavo"), age: 11 };

    println!("{}", gustavo.presentation());
}
