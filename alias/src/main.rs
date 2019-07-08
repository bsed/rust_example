type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;
#[allow(overflowing_literals)]

fn main() {
    println!("Hello, world!");
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
    let decimal = 65.4321_f32;
    //let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is : {}", 1000 as u8);

    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
