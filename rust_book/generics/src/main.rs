#[derive(Debug)]

struct Foo<T> {
    x: T,
    y: T
}

fn main() {
    let bar = Foo::<i32> { x: 6, y: 88 };
    println!("{} and {}", bar.x, bar.y);
}
