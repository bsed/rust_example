use std::num::ParseIntError;


#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _               => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None        => None,
        Some(food)  => match have_recipe(food) {
            None        => None,
            Some(food)  => Some(food),
        }
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

#[derive(Debug)] enum Food2 { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food2);
#[derive(Debug)] struct Chopped(Food2);
#[derive(Debug)] struct Cooked(Food2);

fn peel(food: Option<Food2>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// 切食物。如果没有食物，就返回 `None`。否则返回切好的食物。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// 烹饪食物。这里，我们使用 `map()` 来替代 `match` 以处理各种情况。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// 这个函数会完成削皮切块烹饪一条龙。我们把 `map()` 串起来，以简化代码。
fn process(food: Option<Food2>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// 在尝试吃食物之前确认食物是否存在是非常重要的！
fn eat2(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

type AliasedResult<T> = Result<T, ParseIntError>;

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

// fn multiply4(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     let first_number = try!(first_number_str.parse::<i32>());
//     let second_number = try!(second_number_str.parse::<i32>());

//     Ok(first_number * second_number)
// }

fn multiply5(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number) 
    })
}

fn print5(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn multiply6(first_number_str: &str, second_number_str: &str) ->Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


fn multiply7(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
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

    // print(multiply4("10", "2"));
    // print(multiply4("t", "2"));

    print5(multiply5("10", "2"));
    print5(multiply5("t", "2"));

    //let i: () = "t".parse::<i32>();

    let twenty6 = multiply6("20", "2");
    print(twenty6);

    let tt = multiply6("t", "2");
    print(tt);

    let twenty7 = multiply7("30", "2");
    print(twenty7);

    let tt7 = multiply7("t", "2");
    print(tt7);

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);

    let apple = Some(Food2::Apple);
    let carrot = Some(Food2::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    // 现在让我们试试看起来更简单的 `process()`。
    let cooked_potato = process(potato);

    eat2(cooked_apple);
    eat2(cooked_carrot);
    eat2(cooked_potato);
}
