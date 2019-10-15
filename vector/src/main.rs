fn main() {
    let mut pack = vec![1, 2, 3, 4];

    pack.push(49);
    pack.remove(1);

    for numbers in pack.iter() {
        println!("{}", numbers);
    }
}
