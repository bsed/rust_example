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


fn main(){
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

     let pair = Pair (Box::new(1), Box::new(2));
     pair.destory();

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper2 = 1000;

    let mut acc2 = 0;
    for n2 in 0.. {
        let n_squared = n2 * n2;
        if n_squared >= upper2 {
            break;
        }else if is_odd2(n_squared){
            acc2 += n_squared;
        }
    }
    println!("imperative style: {}", acc2);

    let sum_of_squared_odd_numbers2: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n| n < upper2)
            .filter(|&n| is_odd2(n))
            .fold(0, |sum, i| sum + i);
    println!("functional style: {}", sum_of_squared_odd_numbers2);
}