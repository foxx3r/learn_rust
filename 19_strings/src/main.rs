fn main() {
    let mut my_string = String::from("How's it going? My name is Gabriel");

    println!("length of my string: {}", my_string.len());
    println!("String is empty? {}", my_string.is_empty());
    
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Does the string contains 'Gabriel'? {}", my_string.contains("Gabriel"));

    my_string = String::from("Hello");
    my_string.push_str(", World!");

    println!("{}", my_string);
}
