use crate::examples::hello_world::hello_world;
use crate::examples::struct_enum_const::Point;
use crate::examples::struct_enum_const::Rectangle;

mod examples;
mod struct;
mod fn_ownership;
mod generic_in_rust;
mod guess_num;
mod iter_impl;
mod primitive_type;
mod rust_control_flow;
mod struct_and_enum;
mod thread_test;

fn main() {
    let rectangle = Rectangle::new(1.5, 3.5, 4.5, 5.5);
    let p = Point {
        x: 1.5f32,
        y: 2.5f32,
    };
    let x = examples::struct_enum_const::rect_area(&rectangle);
    let square = examples::struct_enum_const::square(p, 1.5);
    println!("{:#?}", rectangle);
    println!("the area is: {}", x);
    println!("{:#?}", square);
    println!(
        "the area is: {}",
        examples::struct_enum_const::rect_area(&square)
    );
}