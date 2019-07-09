
pub fn data_type {
  let float_num: f32 = 1.001;
  let true_type: bool = true;
  let cat: char = '🐱';
  let a = (1, 2, 3);
  println!("Tuple's first number: {}", a.0);
  println!("Tuple's second number: {}", a.1);
  println!("Tuple's third number: {}", a.2);

  let (x, y, z) = a;
  println!("x={} , y={} , z={}", x, y, z);

  let animal_char: [char; 3] = ['🐕', '🐱', '🐏'];
  let cat_face = animal_char[1];
  let dog_face = animal_char[0];
  println!("Dog --> {}", dog);
  let dog_face = "dog";
}

