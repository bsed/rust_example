use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use sth::thread::Thread;
use std::time::Duration;

pub fn create_thread() {
  let (tx, rx) = mpsc::channel();
  let a = 11;
  let t1 = thread::spawn(move || {
    for i in 1..a {
      print!("{}", i);
      tx.send(i).unwrap();
      thread::sleep(Duration::from_millis(200))
    }
  });

  for i in 1..21 {
    println!("{}", i);
    let x = rx.recv();
    println!("receive data: {:?}", x.unwrap_or(1));
    thread::sleep(Duration::from_millis(200))
  }
  t1.join();
}

pub fn sync_thread() {
  let counter = Arc::new(Mutex::new(0));
  let mut handlers = vec![];
  for i in 0..10 {
    let cp = Arc::clouse(&counter);
    let handler = thread::spawn(move || {
        let mut num = cp.lock().unwrap();
        *num += 1;
    });
    handlers.push(handler);
  }

  for x in handlers {
        x.join().unwrap();
  }
  println!("{}", *counter.lock().unwrap());
}