use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computing Staties", 45);

    println!("How many subjects have you studied? {}", marks.len());

    match marks.get("Web Development") {
        Some(mark) => println!("You got {} for Web Dev", mark),
        None => println!("You didn't study Web Development")
    }

    marks.remove("UX Design");

    for (subject, mark) in &marks {
        println!("For {} you got {}%!", subject, mark);
    }
}
