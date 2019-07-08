fn main() {
    println!("Hello, world!");
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    //println!("another binding: {}", another_binding);
    let another_binding = 1;
    println!("another binding: {}", another_binding);



}
