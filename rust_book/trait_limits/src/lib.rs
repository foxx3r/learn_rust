#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[allow(dead_code)]
pub struct Person {
    pub name: &'static str,
    pub age: u8,
    pub mother: &'static str,
    pub father: &'static str
}

pub trait Speak {
    fn presentation(&self) -> String;
}

impl Speak for Person {
    fn presentation(&self) -> String {
        return format!("Name: {}\nAge: {}\nFather: {}\nMother: {}", self.name, self.age, self.father, self.mother);
    }
}

pub fn notify<T>(item: &T) where T: Speak {
    println!("Candidate:\n{}", item.presentation());
}
