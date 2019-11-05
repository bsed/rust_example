fn main() {
    println!("{}, {}!", "Hello", "world"); // Hello, world!
    println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    println!("{greeting}, {name}!", greeting = "Hello", name = "world"); // Hello, world!
    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]);

    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!
    let y = String::from("Hello, ") + "world!";
    println!("{}", y); // Hello, world!

    let b = plus_one;
    println!("b: {}", b(3)); // b: 4
    let c = b(5);
    println!("c: {}", c); // c: 6
    let b: fn(i32) -> i32 = plus_one; // 会覆盖之前定义的b
    println!("b function: {}", b(3)); // b function: 4
    let c = b(5);
    println!("c function: {}", c); // c function: 6

    let b = plus_one;
    println!("b function ref: {}", plus_three(b, 2)); // b function ref: 4

    let aa = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
    let mut b = [1, 2, 3];
    let cc: [i32; 3] = [1, 2, 3]; //[Type; NO of elements]
    let ee: [i32; 0] = []; //empty array
    println!("{:?}", aa); //[1, 2, 3]
    println!("{:#?}", aa);

    println!("----------------");
    let a = (1, 1.5, true, 'a', "Hello, world!");
    println!("{:?}", a); //(1, 1.5, true, 'a', "Hello, world!")

    // a.0 = 1, a.1 = 1.5, a.2 = true, a.3 = 'a', a.4 = "Hello, world!"

    let b: (i32, f64) = (1, 1.5);
    println!("b:{:?}", b); // b:(1, 1.5)
    println!("b: {:#?}", b);
    /*
    b: (
    1,
    1.5,
    )
    */
    let (c, d) = b;
    println!("c = {:?}, d = {:?}", c, d); // c = 1, d = 1.5

    let (e, _, _, _, f) = a;
    println!("{:?}, {:?}", e, a); // 1, (1, 1.5, true, 'a', "Hello, world!")

    let g = (0,); //single-element tuple
    println!("g:{:?}", g); // g:(0,)

    let h = (b, (2, 4), 5);
    println!("h:{:?}", h); // h:((1, 1.5), (2, 4), 5)

    println!("----------------");

    let team_size = 7;
    if team_size < 5 {
        println!("Small");
    } else if team_size < 10 {
        println!("Medium");
    } else {
        println!("Large");
    }
    // partially refactored code
    let team_size = 7;
    let team_size_in_text;
    if team_size < 5 {
        team_size_in_text = "Small";
    } else if team_size < 10 {
        team_size_in_text = "Medium";
    } else {
        team_size_in_text = "Large";
    }
    println!("Current team size : {}", team_size_in_text);
    //optimistic code
    let team_size = 7;
    let team_size_in_text = if team_size < 5 {
        "Small" //no ;
    } else if team_size < 10 {
        "Medium"
    } else {
        "Large"
    };
    println!("Current team size : {}", team_size_in_text);
    let is_below_eighteen = if team_size < 18 { true } else { false };

    /**
    *  Medium
       Current team size : Medium
       Current team size : Medium
    */

    println!("----------------");
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S",      // check 16
        17 | 18 => "M", // check 17 and 18
        19...21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };
    println!("{}", tshirt_size); // L
    let is_allowed = false;
    let list_type = match is_allowed {
        true => "Full",
        false => "Restricted",
    };
    println!("{}", list_type); // Restricted
    let marks_paper_a: u8 = 25;
    let marks_paper_b: u8 = 30;
    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard",
    };
    println!("{}", output); // Work hard
    /**
    *
    *  L
       Restricted
       Work hard
    */

    println!("----------------");

    let mut a = 1;
    while a <= 10 {
        println!("Current value : {}", a);
        a += 1; //no ++ or -- in Rust
    }
    // Usage of break and continue
    let mut b = 0;
    while b < 5 {
        if b == 0 {
            println!("Skip value : {}", b);
            b += 1;
            continue;
        } else if b == 2 {
            println!("Break At : {}", b);
            break;
        }
        println!("Current value : {}", b);
        b += 1;
    }
    // Outer break
    let mut c1 = 1;
    'outer_while: while c1 < 6 {
        //set label outer_while
        let mut c2 = 1;
        'inner_while: while c2 < 6 {
            println!("Current Value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 {
                break 'outer_while;
            } //kill outer_while
            c2 += 1;
        }
        c1 += 1;
    }

    /**
     *
    Current value : 1
    Current value : 2
    Current value : 3
    Current value : 4
    Current value : 5
    Current value : 6
    Current value : 7
    Current value : 8
    Current value : 9
    Current value : 10
    Skip value : 0
    Current value : 1
    Break At : 2
    Current Value : [1][1]
    Current Value : [1][2]
    Current Value : [1][3]
    Current Value : [1][4]
    Current Value : [1][5]
    Current Value : [2][1]
    Current Value : [2][2]
     */


    println!("----------------");
    // loop {
    //     println!("Loop forever!");
    // }
    // // Usage of break and continue
    // let mut a = 0;
    // loop {
    //     if a == 0 {
    //         println!("Skip Value : {}", a);
    //         a += 1;
    //         continue;
    //     } else if a == 2 {
    //         println!("Break At : {}", a);
    //         break;
    //     }
    //     println!("Current Value : {}", a);
    //     a += 1;
    // }
    // // Outer break

    // let mut b1 = 1;
    // 'outer_loop: loop {
    //     //set label outer_loop
    //     let mut b2 = 1;
    //     'inner_loop: loop {
    //         println!("Current Value : [{}][{}]", b1, b2);
    //         if b1 == 2 && b2 == 2 {
    //             break 'outer_loop; // kill outer_loop
    //         } else if b2 == 5 {
    //             break;
    //         }
    //         b2 += 1;
    //     }
    //     b1 += 1;
    // }

    /**
     * 
Loop forever!
Loop forever!
Loop forever!
     */

println!("----------------");
    for a in 0..10 { //(a = 0; a <10; a++) // 0 to 10(exclusive)
  println!("Current value : {}", a);
}
// Usage of break and continue
for b in 0..6 {
  if b == 0 {
    println!("Skip Value : {}", b);
    continue;
  } else if b == 2 {
    println!("Break At : {}", b);
    break;
  }
  println!("Current value : {}", b);
}
// Outer break
'outer_for: for c1 in 1..6 { //set label outer_for
  'inner_for: for c2 in 1..6 {
    println!("Current Value : [{}][{}]", c1, c2);
    if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
  }
}
// Working with arrays/vectors
let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];
for n in 0..group.len() { //group.len() = 4 -> 0..4  check group.len()on each iteration
  println!("Current Person : {}", group[n]);
}
for person in group.iter() { // group.iter() turn the array into a simple iterator
  println!("Current Person : {}", person);
}
}

fn print_sum(a: i8, b: i8) {
    println!("sum is {}", a + b);
}

fn plus_one(a: i32) -> i32 {
    a + 1 // 不加分号，等同于 return a + 1
}

fn plus_two(a: i32) -> i32 {
    return a + 2;
}

fn plus_three(b: fn(i32) -> i32, x: i32) -> i32 {
    b(b(x))
}
