#[allow(dead_code)]

struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_equal(&self) -> bool {
        return self.width == self.height;
    }
}

fn main() {
    println!("Hello, world!");
}

fn function_test() -> i32 {
    return 2;
}

#[cfg(test)]
mod to_test {
    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(3 != 4);
        panic!("Oh no");
    }

    #[test]
    fn test_functions() {
        assert_eq!(super::function_test(), 2);
    }

    #[test]
    fn test_struct() {
        let r = super::Rectangle {
            width: 50,
            height: 87
        };

        assert_eq!(r.is_equal(), true);
    }
}
