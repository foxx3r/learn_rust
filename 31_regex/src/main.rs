extern crate regex;

use regex::Regex;
use std::io;
use std::io::Write;

fn main() {
    let re = Regex::new(r"\d{5}-\d{4}").unwrap();
    let mut expression = String::new();

    print!("Type a number like this format \
    87342-9902 -> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut expression)
        .expect("An error has occured");
    println!("");

    match re.is_match(&expression) {
        true => println!("This value is true"),
        false => println!("This value is false")
    }
}
