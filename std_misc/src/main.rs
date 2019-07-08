use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}", args.len()-1, &args[1..]);
}
