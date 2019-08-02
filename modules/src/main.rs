mod hello;
mod sub;
use sub::module::say_hello_submodule;

fn main() {
    println!("Hello, world!");
    let mut s = String::from("kelvin");
    hello::say_hello(&mut s);
    say_hello_submodule();
}
