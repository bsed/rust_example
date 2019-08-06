use std::sync:: {Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, world!");
    let mutex = Arc::new(Mutex::new(0));

    let c_mutex = Arc::clone(&mutex);
    let t1 = thread::spawn(move || {
        let mut count = c_mutex.lock().unwrap();
        println!("-----------------------------");
        println!("t1 got the lock, will increment count (current value: {})", count);
        *count += 1;
        println!("t1 updated count: {}", count);
        println!("-----------------------------");
    });

    let c_mutex = Arc::clone(&mutex);
    let t2 = thread::spawn(move || { // t2
        let mut count = c_mutex.lock().unwrap();
        println!("-----------------------------");
        println!("t2 got the lock, will increment count (current value: {})", count);
        *count += 1;
        println!("t2 updated count: {}", count);
        println!("-----------------------------");
    });

    let mut handles = vec![];
    handles.push(t1);
    handles.push(t2);

    for i in 2..1000000 {
        let c_mutex = Arc::clone(&mutex);
        let t = thread::spawn(move || {
            let mut count = c_mutex.lock().unwrap();
            println!("-----------------------------");
            println!("t{} got the lock, will increment count (current value: {})", i, count);
            *count += 1;
            println!("t{} updated count: {}", i, count);
            println!("-----------------------------");
        });
        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let count = mutex.lock().unwrap();
    println!("Hello, concurrency! {}", count);
}
