#[derive(Debug)]
struct NormalFoo {
    id: i32,
}

#[derive(Debug, Copy, Clone)]
struct CopyFoo {
    id: i32,
}

#[derive(Debug, Copy, Clone)]
struct AnotherCopyFoo {
    name: &'static str,
}

fn main() {
    let a = NormalFoo { id: 1 };
    println!("{:?}", a);

    let b = a;
    // println!("{:?}", a); 
    println!("{:?}", b); 

    let c = CopyFoo {id: 2};
    println!("{:?}", c);
    
    let d = c;
    println!("{:?}", c);
    println!("{:?}", d);

    let e = AnotherCopyFoo { name: "e" }; 
    println!("{:?}", e);

    let f = e;
    println!("{:?}", e); 
    println!("{:?}", f);

}
