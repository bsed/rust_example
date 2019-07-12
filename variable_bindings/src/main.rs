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

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}
