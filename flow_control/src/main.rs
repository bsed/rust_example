enum Foo {
    Bar,
    Baz,
    Qux(u32)
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
}
