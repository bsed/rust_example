use std::collections::LinkedList;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    let mut mutex: Arc<Mutex<LinkedList<u32>>> = Arc::new(Mutex::new(list));
    let mut threads = vec![];
    threads.push(create_search_thread(&mut mutex, "ST1"));
    threads.push(create_search_thread(&mut mutex, "ST2"));
    threads.push(create_insert_thread(&mut mutex, "IT1"));
    threads.push(create_insert_thread(&mut mutex, "IT2"));

    for thread in threads {
        thread.join().unwrap();
    }

    let list = mutex.clone();
    let locked_list = list.lock().unwrap();
    println!("{:?}", *locked_list)
}

fn create_insert_thread(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        let mut locked_list = list.lock().unwrap();
        println!("Inserter thread {}: {:?}", name, *locked_list);
        locked_list.push_back(999);
    });
    return t;
}

fn create_search_thread(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        let locked_list = list.lock().unwrap();
        println!("Searcher thread {}: {:?}", name, *locked_list);
        for item in locked_list.deref() {
            println!("Searcher thread {}: {}", name, item);
        }
    });

    return t;
}