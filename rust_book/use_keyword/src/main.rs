pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() { }
        }
    }
}

enum TrafficLight {
    Yellow,
    Red,
    Green
}

use a::series::of;
use TrafficLight::{Yellow, Red};

#[allow(unused_variables)]
fn main() {
    of::nested_modules();
    let yellow = Yellow;
    let red = Red;
    let green = TrafficLight::Green;
}
