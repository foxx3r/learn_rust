struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}
fn main() {
    let pyramid = Rectangle { width: 15, height: 5 };

    pyramid.print_description();

    println!("width of rectangle is equal yourself height? {}", pyramid.is_square());
}
