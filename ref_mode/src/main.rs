#![feature(core_intrinsics)]

fn main() {
    println!("Hello, world!");

    let s = String::from("hello");
    let sref1 = &s;
    let ref sref2 = s;

    println!("{:p}", sref1);
    println!("{:p}", sref2);

    f1(&s);
    f2(s);

    let x = &false;
    print_type_name_if(x);

    let &x = &false;
    print_type_name_if(x);

    let ref x= &false;
    print_type_name_if(x);

}

fn f1(_s: &String) {
    println!("{:?}", _s);
}

fn f2(ref _s: String) {
    println!("{:p}", _s);
}

fn print_type_name_if<T>(_: T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() })
}
