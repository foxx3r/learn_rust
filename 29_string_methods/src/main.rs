use std::io;
use std::io::Write;

fn main() {
    /* replace */
    {
        let my_string = String::from("Rust is fantastic!");
        println!("{}", my_string.replace("fantastic", "great"));
    }
    /* lines */
    {
        let my_string = String::from("The weather is\nnice\noutside mate");
        
        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }
    /* split */
    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        for token in tokens.iter() {
            print!("{} ", token);
            io::stdout().flush().unwrap();
        }
        println!("");
    }
    /* trim */
    {
        let my_string = String::from("  My name is Gabriel!  \n\n");

        println!("Before the trim: {}", my_string);
        println!("After the trim: {}", my_string.trim());
    }
    /* chars */
    {
        let my_string = String::from("foxx3r on GitHub");
        println!("{}", my_string);

        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4")
        }
    }
}
