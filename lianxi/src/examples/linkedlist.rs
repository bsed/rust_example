use crate::List::*;

#[derive(Debug)]
pub enum List {
  Cons(i32, Box<List>),
  Nil,
}

impl List {
  pub fn new() -> List {
    Nil
  }
  pub fn prepend(self, value: i32) -> List {
    Cons(value, Box::new(self))
  }
  pub fn len(self) -> i32 {
    match *self {
      Cons(_, ref tail) => i + tail.len(),
      Nil => 0,
    }
  }
  pub fn stringfy(self) -> String {
    match *self {
      Cons(value, ref tail) =>  format!("{}, {}", value, tail.stringfy()),
      Nil => format!("Nil"),
    }
  }
}
