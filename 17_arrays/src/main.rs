fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // arrays can't have 2 or more data types
    // this array have all of elements as i32
    // and 5 elements

    for index_of_element in 0..numbers.len() {
        println!("{}", numbers[index_of_element]);
    }

    let points = [2; 400];

    for point in points.iter() {
        println!("{}", point);
    }
}
