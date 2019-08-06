use std::rc::Rc;

#[derive(Debug)]
struct bla {}

impl bla {
    fn new() -> bla {
        bla {}
    }
}

fn main() {
    let mut x = "value";
    println!("x:{}", x);
    x = "mutated";
    println!("x after mutation: {}", x);

    let y = &mut x;
    println!("y: {}", y);

    //println!("x: {}", x);
    // let z = &mut x;
    // println!("z: {}", z);

    *y = "mutated by y";
    println!("y after mutation: {}", y);
    println!("x after mutation: {}", x);

    x = "mutated by x";
    println!("x after mutation: {}", x);

    let mut x = "value";
    println!("x:{}", x);
    x = "mutated";
    println!("x after mutation: {}", x);

    {
        let y = &mut x;
        println!("y: {}", y);

        *y = "mutated by y";
        println!("y after mutation: {}", y);
        println!("x after mutation: {}", x);
    }

    println!("x: {}", x);

    let five = 5;
    let a = &five;
    let b = &five;
    println!("a:{} b:{}", a, b);

    let mut five = Rc::new(5);
    let a = Rc::new(Rc::clone(&mut five));
    let b = Rc::new(Rc::clone(&mut five));
    println!("a:{} b:{}", a, b);

    let bla = bla::new();
    println!("{:?}", bla);
    drop(bla);
    //println!("{:?}", bla);

    let six = 6;
    drop(six); // this does not drop six, because primitive types does not implement the Drop trait
    println!("{}", six); // this works
}
