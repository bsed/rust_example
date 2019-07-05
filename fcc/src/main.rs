
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destory(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn is_odd2(n: u32) -> bool {
    n % 2 == 1
}

// fn foo3() -> ! {
//     panic!("This call never returns.");
// }

fn some_fn3() {
    ()
}

fn apply5<F>(f: F)
where
    F: FnOnce(),
{
    f();
}


fn call_me7<F: Fn()>(f: F) {
    f()
}
fn function7() {
    println!("I'm a function!");
}

fn apply8<F>(f: F) where
    F: FnOnce() {
        f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
    }

fn create_fn9() -> Box<dyn Fn()> {
    let text = "Fn".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut9() -> Box<dyn FnMut()> {
    let text = "FnMut".to_owned();
    Box::new(move || println!("This is b: {}", text))
}


fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destory();

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper2 = 1000;

    let mut acc2 = 0;
    for n2 in 0.. {
        let n_squared = n2 * n2;
        if n_squared >= upper2 {
            break;
        } else if is_odd2(n_squared) {
            acc2 += n_squared;
        }
    }
    println!("imperative style: {}", acc2);

    let sum_of_squared_odd_numbers2: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper2)
        .filter(|&n| is_odd2(n))
        .fold(0, |sum, i| sum + i);
    println!("functional style: {}", sum_of_squared_odd_numbers2);

    let _a3: () = some_fn3();
    println!("This function returns and you can see this line.");

    // let x3: ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    fn sum_odd_numbers3(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i % 2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers3(9)
    );

    fn function4(i: i32) -> i32 {
        i + 1
    }
    let closure_annotated4 = |i: i32| -> i32 { i + 1 };
    let closure_inferred4 = |i| i + 1;

    let i = 1;
    println!("function4: {}", function4(i));
    println!("closure_annotated4: {}", closure_annotated4(i));
    println!("closure_inferred4: {}", closure_inferred4(i));

    let one4 = || 1;
    println!("closure returning one4: {}", one4());

    let x5 = 7;
    let print5 = || print!("{}", x5);
    apply5(print5);

    let color6 = "green";
    let print6 = || println!("`color`: {} ", color6);
    print6();
    print6();

    let mut count6 = 0;
    let mut inc6 = || {
        count6 += 1;
        println!("`count`: {}", count6);
    };
    inc6();
    inc6();

    //let reborrow = &mut count6;
    let movable6 = Box::new(3);
    let consume6 = || {
        println!("`movable`: {:?}", movable6);
        std::mem::drop(movable6);
    };

    consume6();

    let closure7 = || println!("I'm a closure!");
    call_me7(closure7);
    call_me7(function7);

    let greeding8 = "hello";
    let mut farewell8 = "goodbye".to_owned();

    let diary8 = || {
        println!("I said {}.", greeding8);

        farewell8.push_str("!!!");
        println!("Then I screamed {}.", farewell8);
        println!("Now I can sleep. zzzzz");

        std::mem::drop(farewell8);
    };

    apply8(diary8);
    let double = |x| 2*x;
    println!("3 doubled: {}", apply_to_3(double));

    let fn_plain9 = create_fn9();
    let mut fn_mut9 = create_fnmut9();

    fn_plain9();
    fn_mut9();

    let vec91 = vec![1, 2, 3];
    let vec92 = vec![4, 5, 6];

    println!("2 in vec91: {}", vec91.iter().any(|&x| x == 2));
    print!("2 in vec92: {}", vec92.into_iter().any(|x| x == 2));

}
