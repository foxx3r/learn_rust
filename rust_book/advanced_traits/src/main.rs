trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("I'll fly!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("I can fly!");
    }
}

impl Human {
    fn fly(&self) {
        println!("I'm very angry cuz I can't fly!");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
