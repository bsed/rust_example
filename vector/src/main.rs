
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2){
        Some(thir) => println!("The third element is {}", thir),
        None => println!("There is no third element."),
    }

    let first = &v[0];
    println!("The first element is: {}", first);

    let mut v4 = vec![2, 333, 444 ,23, 22, -1, 11];
    for i in &mut v4 {
        *i += 1;
        println!("i {}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.33),
    ];

}
