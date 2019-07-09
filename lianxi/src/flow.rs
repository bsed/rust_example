use std::time::SystemTime;

fn control_flow(max: i32) {
  if max > 0 {
    println!("{} > 0", max);
  }

  let a = if max > 0 {
    1
    } else {
      -1
    };
  println!("let value: {}", a);

  let mut conter = 0;
  let result = loop {
    counter += 1;
     if counter % 2 == 1 {
            continue;
        }
        println!("counter: {}", counter);
        if counter == 10 {
            break counter * 2;
        }
  };
  println!("result is: {}", result);
  assert_eq!(result, 20);

  while counter != 0 {
    println!("Now, counter is: {}", counter);
    counter -= 1;
  }
  let arr:[i23; 6] = [1, 2, 3, 4, 5];
  for i in arr.iter() {
    println!("number is: {}", i);
  }
  for number in (1..11).rev() {
    println!("number ===> {}", number);
  }
}

pub fn bubble() {
  let mut arr = [10000; 10000];
  for i in 1..arr.len() {
    arr[i] = arr.len() - i;
  }
  for i in arr.iter() {
        println!("{}", i);
  }

  let len = arr.len();
  let start = SystemTime::now();
  for i in 0...len - 1 {
    for j in i + 1..len {
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
  }

  let end = start.elapsed().expect("SystemTime: elapsed failed");
  println!("hello {:#?}", end.as_millis());
}

pub fn match_test() {
  let x = Some(5);
  let y = 7;
  let n = true;
  match x {
    Some(40) => println!("{}", 40);
    Some(y) => println!("{}", y),
    _ => println!("Default value: {}", y);
  }

  println!("{:?}   {:?}", x, y);
  match y {
    4 | 5 | 6 if n => println!("yes");
    _ => println!("false");
  }
}