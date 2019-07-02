enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn age() -> i32 {
    15
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    println!("Hello, world!");

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let names2 = vec!["Bob2", "Frank2", "Ferris2"];
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us3!"),
            _ => println!("Hello 3{}", name),
        }
    }

    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, half the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);

    let number3 = Some(7);
    let letter3: Option<i32> = None;
    let emoticon3: Option<i32> = None;
    if let Some(i) = number3 {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter3 {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    };

    let i_like_letters = false;
    if let Some(i) = emoticon3 {
        println!("Matched {:?}!", i);
    }else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    }else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    };


    let a4 = Foo::Bar;
    let b4 = Foo::Baz;
    let c4 = Foo::Qux(100);

    if let Foo::Bar = a4 {
        println!("a is foobar");
    }

    if let Foo::Bar = b4 {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c4 {
        println!("c is {}", value);
    }

    let mut count = 0u32;
    println!("Let's count unitl infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 跳过这次迭代的剩下内容
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // 退出循环
            break;
        }
    }

    let number4 = 13;
    println!("Tell me about {}", number4);

    match number4 {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13...19 => println!("a teen"),
        _ => println!("Ain't special"),
    }

    let boolean4 = true;
    // match 也是一个表达式
    let binary4 = match boolean4 {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean4, binary4);

    let mut n5 = 1;
    while n5 < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        }else if n % 3 == 0 {
            println!("fizz");
        }else if n % 5 == 0 {
            println!("buzz");
        }else {
            println!("{}", n);
        }

        n5 += 1;
    }

    let mut optional6 = Some(0);
    loop {
        match optional6 {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional6 = None;
                } else {
                    println!("`i` is `{:?}`. Try again. ", i);
                    optional6 = Some(i + 1);
                }
            },
            _ => {break;}
        }
    }
    
    let mut optional7 = Some(0);
    while let Some(i) = optional7 {
        if i > 9 {
            println!("Greater that 9, quit!");
            optional7 = None;
        }
        else {
            println!("`i` is `{:?}`. Try again.", i);
            optional7 = Some(i + 1);
        }
    }

    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }
        println!("this point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter9 = 0;
    let result9 = loop {
        counter9 += 1;
        if counter9 == 10 {
            break counter9 * 2;
        }
    };

    assert_eq!(result9, 20);

    println!("Tell me type of person you are");
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1 ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    let pair10 = (2, -2);
    println!("Tell me about {:? }", pair10);
    match pair10 {
        (x, y) if x == y => println!("These are twins"),
         (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    let color11 = Color::RGB(122 , 17, 40);
     println!("What color is it?");
     match color11 {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
     }
}
