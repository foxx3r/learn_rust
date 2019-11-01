use std::cell::RefCell;
use std::borrow::Borrow;

#[derive(Debug)]
struct Message {
    message: RefCell<Vec<String>>
}

impl Message {
    fn new() -> Message {
        return Message { message: RefCell::new(vec![]) };
    }
    fn send(&self, msg: &str) {
        self.message.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    let x = Message::new();
    x.send("Hi, what's up?");
    println!("{:#?}", x.borrow());
}
