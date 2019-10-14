fn main() {
    let x = 20;
    {
        let y = 40;

        println!("20 X 40 = {}", x * y);
    }
    let y = 80;

    println!("20 X 80 = {}", x * y);
}
