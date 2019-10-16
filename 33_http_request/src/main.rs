extern crate reqwest;

fn main() {
    // first method 
    match reqwest::get("http://youtube.local/hello") {
        Ok(mut response) => {
            // Check if 200 OK
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not response text")
                }
            } else {
                println!("Response was not 200 OK.")
            }
        },
        Err(_) => println!("Could not make the request!")
    }

    // second method
    let response_text = reqwest::get("https://github.com/foxx3r").expect("Couldn't make request")
        .text().expect("Could not read the response text!");
}
