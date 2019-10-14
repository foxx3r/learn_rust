fn main() {
    let foo = String::from("I am your prey");
    eat_string(foo); // foo is not valid


    let mut foo: i32 = 8;
    eat_number(foo);
    println!("{}", foo);

    // references
    let bar = String::from("I am a bear");
    eat_reference_to_string(&bar);
    println!("{}", bar); // is valid

    let my_string = &bar; // without &, bar would be invalid
    println!("{}", bar);
    println!("{}", my_string);

    // mutable references
    let mutable_reference = &mut foo;
    *mutable_reference = 20;
    println!("{}", foo);
}

fn eat_string(prey: String) {
    drop(prey);
}

fn eat_number(prey: i32) {
    drop(prey);
}

fn eat_reference_to_string(prey: &String) {
    drop(prey);
}
