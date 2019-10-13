fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        
        if counter == 7 {
            continue;
        }

        if counter == 11 {
            break;
        }
        println!("{}", counter);
    }
}
