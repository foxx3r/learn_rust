use std::process::Command;

fn main() {
    let mut cmd = Command::new("uname");
    cmd.arg("-r");

    match cmd.output() {
        Ok(output) => {
            println!("Your current version of kernel: {}", String::from_utf8_lossy(&output.stdout))
        },
        Err(e) => println!("Failed to exec the command: {}", e)
    }
}
