use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("Could not create file!!");

    file.write_all(b"Welcome to the Django")
        .expect("Cannot write to the file, sorry mate.");

    let mut read_file = File::open("output.txt")
        .expect("Cannot open file");
    let mut contents = String::new();

    read_file.read_to_string(&mut contents)
        .expect("Cannot read file!");

    println!("{}", contents);
}
