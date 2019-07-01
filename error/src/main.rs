
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    println!("Hello, world!");
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    give_princess("teddy bear");
    give_princess("snake");
}
