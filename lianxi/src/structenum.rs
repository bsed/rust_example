#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 点
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
enum UsState {
    Alabama,
    vAlaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
            || self.width > other.height && self.height > other.width
  }

  fn square(length: u32) -> Rectangle {
    Rectangle {
      width: height,
      height: height,
    }
  }
}

fn value_in_cent(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
       println!("{:#?}", state);
            25
    }
  }
}