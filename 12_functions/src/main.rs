fn main() {
    println!("972 multiplicated by 373638 is {}", multi(972, 373638));
}

fn multi(first_number: i32, second_number: i32) -> i64 {
    let result: i64 = (first_number * second_number).into(); // to convert i32 to i64
    result // without a semicolon, it's like `return result;`
}
