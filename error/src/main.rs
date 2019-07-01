
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn give_commoner(gift: Option<&str>){
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess2(gift: Option<&str>){
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }
    println!("I love {}s!!!!!", inside);

}

fn main() {
    println!("Hello, world!");
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    give_princess("teddy bear");
    give_princess("snake");

    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

     let bird = Some("robin");
    let nothing = None;

    give_princess2(bird);
    give_princess2(nothing);
}
