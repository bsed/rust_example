use env_logger::fmt::Color::Magenta;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
struct Matix(f32, f32, f32, f32);

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (a, b) = pair;
  (b, a)
}

impl Display for Matix {
  fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
    write!(
      f,
      "({:.1} {:.1})\n({:.1} {:.1})",
      self.0, self.1, self.2, self.3
    )
  }
}

pub fn primitive_type() {
  let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10;
  println!("{}", long_tuple.1);
  let tuple_of_tuples = ((1, 2), (3, 4));
  println!("{:?}", tuple_of_tuples);
  println!("{:?}", long_tuple);
  println!("{:?}", reverse((21, true)));
  println!("one element tuple: {:?}", (5u32,));
  println!("just a integer: {:?}", (5u32));

  let destructured_tuple = (1.0, 2.0, 3.0, 4.0);
  let (a, b, c, d) = destructured_tuple;
  println!("{:?} {:?} {:?} {:?}", a, b, c, d);
  let matix = Matix(1.0, 2.0, 3.0, 4.0);
  println!("{}", matix);

  let mut arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  println!("Array's length: {}", arr.len());
  for i in &arr[0..5] {
      println!("{}", i);
  }
  for i in &arr {
        println!("{}", i);
    }
}

