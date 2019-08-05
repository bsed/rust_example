use std::thread;
use std::time::Duration;


fn main() {
    println!("Hello, world!");
    let handle = thread::spawn(|| {
        for i in 1..1000000 {
            println!("hi number {} from the spawned thread! ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }


    handle.join().unwrap();
}