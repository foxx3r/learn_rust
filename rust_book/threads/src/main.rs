use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let x: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];

    let new_handle = thread::spawn(move || {
        println!("There's a vector: {:?}", x);
        thread::sleep(Duration::from_millis(1));
    });

    println!("Hello, World!");
    thread::sleep(Duration::from_millis(1));

    new_handle.join().unwrap();
}
