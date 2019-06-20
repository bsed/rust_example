extern crate rary;

fn main() {
    rary::public_function();


    rary::indirect_access();
}


// rustc main.rs --extern rary=library.rlib && ./main