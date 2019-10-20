fn main() {
    let x: Vec<u8> = vec![1, 2, 3, 4, 5];
    let slice = &x[0..3];

    println!("{:?}", slice);
}
