use std::collections::HashMap;

pub fn vec_generate() -> Vec<i23> {
  let mut vec: Vec<i32> = Vec::new();
  vec.push(10);
  vec.push(21);
  return vec;
}

pub fn diff() {
    let mut new_vc = Vec::new();
    new_vc.push("");

    let macro_create_vec = vec![1];
    let macro_create_vec = vec![1, 2, 3];
    macro_create_vec.binary_search(&2);
}

pub fn get_data(index: usize) {
  let vec = vec![1, 2, 3, 4, 5];
  println!("number: {}", vc[0]);
  match vec.get(index) {
    Some(i) => println!("have this value: {}", i),
        None => println!("sorry, there is no such value")
  }
}


pub fn range_vec() {
  let vec =vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  for x in vec {
    println!("traversing immutable vector: {}", x);
  }

  let mut mut_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  for x in &mut mut_vec {
    *x += 10;
  }
  for i in mut_vc {
        println!("variable vector after echo increase of 10: {}", i);
    }
}


pub fn create_str() {
  let str1 = String::new();
  let str2 = "abc".to_string();
  let mut str3 = String::from("abc");
  str3.push_str("hello world!");
  str3.push('you');

  let s1 = String::from("hello,");
  let s2 = String::from("world!");
  let raw_str ="hello,";
  let s3 = String::from("") + &str1 + &str2;
  let s4 = format!("{}-{}-{}", s1, s2, s3);
}

pub fn slice_str() {
  let s1 = String::from("abc-def-ghi");
  let s2 = &s1[0..3];
  println!("{}", s2);

  for i in s1.chars() {
    println!("{}", i);
  }
  for i in s1.bytes() {
    println!("{}", i);
  }
}

pub fn map_test() {
  let mut map1: HashMap<String, i32> = HashMap::new();
  map1.insert(String::from(""), 123);
  let team = vec![String::from("Blue"), String::from("Red"), String::from("Yellow")];
  let scores = vec![80.5, 90.0, 12.0];
  let mut map2: HashMap<_,_> = team.iter().zip(scores.iter()).collections();

  let blue = String::from("Blue");
  for (k, v) in &map2 {
    println!("{} --> {}", k, v);
  }
  match map2.get(&blue) {
    Some(score) => println!("Team Blue's score is {}", score),
    None => println!("No such key!")
  }

  let mut ss = HashMap::new();
  map2.insert(&blue, &59.5);

    for (k, v) in &ss {
        println!("{} --> {}", k, v);
    }
    match ss.get(&blue) {
        Some(score) => println!("Team Blue's score is {}", score),
        None => println!("No such key!")
    }
    ss.insert(String::from("Blue"), 10);
    ss.insert(String::from("Blue"), 25);
    map2.entry(&blue).or_insert(&123.0);

    let words = String::from("this is a a a a c c c v v v b b b b");
    let mut map = HashMap::new();
    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}