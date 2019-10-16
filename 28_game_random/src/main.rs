extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 21);

    println!("Guess the number, it's between 1 and 20");

    loop {
        let mut guess = String::new();

        print!("Type your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the entry");

        let guess: u32 = guess.trim().parse()
            .expect("Please, type a number");
    
        println!("You said: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Very low");
            },
            Ordering::Greater => {
                println!("Very high");
            },
            Ordering::Equal => {
                println!("You're right");
                break;
            }
        }
    }
}
