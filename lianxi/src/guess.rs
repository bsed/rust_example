use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number {
  let secret_num = rand::thread_rng().gen_range(1, 101);
  println!("generate number: {}", secret_num);
  println!("Guess the number!");

  loop {
    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("faile to read line");

    let guessL u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    println!("Your guessed: {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

  }
}