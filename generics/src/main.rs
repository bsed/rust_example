use std::fmt::{Debug, Display};
use std::marker::PhantomData;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}


#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 { 
    t.area() 
}

struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}


struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T,U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}


struct S2;
struct GenericVal2<T>(T,); // 泛型类型 `GenericVal`

impl GenericVal2<f32> {}
impl GenericVal2<S2> {}

impl<T> GenericVal2<T> {}

struct Val2 {
    val: f64
}

impl Val2 {
    fn value(&self) -> &f64 { &self.val }
}

struct GenVal2<T> {
    gen_val: T
}

impl <T> GenVal2<T> {
    fn value(&self) -> &T {&self.gen_val}
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

struct Years4(i64);
struct Days4(i64);
impl Years4 {
    pub fn to_days(&self) -> Days4 {
        Days4(self.0 * 365)
    }
}

impl Days4 {
    pub fn to_years(&self) -> Years4 {
        Years4(self.0 / 365)
    }
}

fn old_enough4(age: &Years4) -> bool {
    age.0 >= 18
}

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B>{first: A, phantom: PhantomData<B> }

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
}


fn main() {
    println!("Hello, world!");

    let rectangle = Rectangle { length: 3.0, height: 4.0};
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

    //empty;
    //null;

    let x2 = Val2 { val: 3.0 };
    let y2= GenVal2 { gen_val: 3i32 };
    
    println!("{}, {}", x2.value(), y2.value());

    let string3 = "words";
    let array3 = [1, 2, 3];
    let vec3 = vec![1, 2, 3];
    compare_prints(&string3);
    compare_types(&array3, &vec3);

    let age4 = Years4(5);
    let age_days4 = age4.to_days();
    println!("Old enough {}", old_enough4(&age4));
    println!("Old enough {}", old_enough4(&age_days4.to_years()));
    //println!("Old enough {}", old_enough4(&age_days4));

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    
    // 编译期错误！类型不匹配，所以这些值不能够比较：
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);
    
    // 编译期错误！类型不匹配，所以这些值不能够比较：
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);

    let vec5 = vec![1, 2, 3];
    vec5.print_in_option();

 
}
