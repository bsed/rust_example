fn main() {
    println!("{}, {}!", "Hello", "world"); // Hello, world!
    println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    println!("{greeting}, {name}!", greeting="Hello", name="world"); // Hello, world!
    println!("{:?}", [1,2,3]); // [1, 2, 3]
    println!("{:#?}", [1,2,3]);

    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!
    let y = String::from("Hello, ") + "world!";
    println!("{}", y); // Hello, world!

    let b = plus_one;
    println!("b: {}", b(3)); // b: 4
    let c = b(5);
    println!("c: {}", c);  // c: 6
    
    let b: fn(i32) -> i32 = plus_one; // 会覆盖之前定义的b
    println!("b function: {}", b(3)); // b function: 4
    let c = b(5);
    println!("c function: {}", c);  // c function: 6

    let b = plus_one;
    println!("b function ref: {}", plus_three(b, 2)); // b function ref: 4

    let aa = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
    let mut b = [1, 2, 3];
    let cc: [i32; 3] = [1, 2, 3]; //[Type; NO of elements]
    let ee: [i32; 0] = []; //empty array
    println!("{:?}", aa); //[1, 2, 3]
    println!("{:#?}", aa);
}


fn print_sum(a: i8, b: i8) {
    println!("sum is {}", a + b);
}

fn plus_one(a: i32) -> i32 {
    a + 1 // 不加分号，等同于 return a + 1
}

fn plus_two(a: i32) -> i32 {
    return a + 2;
}

fn plus_three(b: fn(i32) -> i32, x: i32) -> i32 {
    b(b(x))
}