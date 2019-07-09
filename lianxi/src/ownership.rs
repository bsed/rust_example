fn functions(a: i32, b: i32) -> i32 {
  //return a + b;
  a + b
}

fn fibonacci_number(number: i32) -> i32 {
  if number >= 2 {
    return fibonacci_number(number - 2) + fibonacci_number(number -1);
  } else number == 1 {
    return 1;
  } else {
    return 0;
  }
}

fn ownership_test() {
  let mut str1 = String::from("hello");
  str1.push_str(",kelvin");
  takes_ownership(str1);
  //let str2 = str1.clone(); //deep copy

  let x = 123;
  let y = x;
  makes_copy(x);
  makes_copy(y);
  println!("x: {}, y: {}, str2: {}", x, y, str2);

}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(int_num: i32) {
  println!("{}", int_num);
}

fn calc_len(s: String) -> (String, usize) {
  let len = s.len();
  retrun (s, len);
}


fn str_slice(some_str_slice: &str) -> &str {
  let bytes = some_str_slice.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &some_str_slice[0..i];
    }
  }
  &some_str_slice[..]
}