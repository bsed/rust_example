use std::num::ParseIntError;



fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


fn multiply3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

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
    if inside == "snake" { panic!("AA2Aaaaaa!!!!"); }
    println!("I love {}s!!!!!", inside);

}


fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误 1
    2 * first.parse::<i32>().unwrap() // 生成错误 2
}


fn main() {
    println!("Hello, world!");
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    give_princess("teddy bear");
    give_princess("snake1");

    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

     let bird = Some("robin");
    //let nothing = None;

    give_princess2(bird);
    //give_princess2(nothing);

    let numbers = vec!["42", "93", "18"];
    //let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {}", double_first(numbers));
    //println!("The first doubled is {}", double_first(empty));
    // 错误1：输入 vector 为空
    //println!("The first doubled is {}", double_first(strings));
    // 错误2：此元素不能解析成数字

    let possible_numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", possible_numbers);

    let strings2 = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings2
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);

    let strings3 = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings3
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    print(multiply2("10", "2"));
    print(multiply2("t", "2"));

    print(multiply3("10", "2"));
    print(multiply3("t", "2"));
}
