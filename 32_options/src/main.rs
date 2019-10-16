fn main() {
    println!("Occupation is {}", match get_occupation("Domenic") {
        Some(occupation) => occupation,
        None => "No occupation found!"
    });
}

fn get_occupation(name: &str) -> Option<&str> {
    return match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}
