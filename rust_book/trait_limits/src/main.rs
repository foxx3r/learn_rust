mod lib;

fn main() {
    let john = lib::Person {
        name: "John",
        age: 19,
        mother: "Mary",
        father: "Michael"
    };

    lib::notify(&john);
}
