use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
struct Person {
  name: String,
  age: u32,
}

impl Person {
  pub fn new (name: String, age: u32) -> Person {
    Person {
      name,
      age,
    }
  }
}

impl Display for Person {
  fn fmt(&self, f: &mut Formatter) ->Result<(), Error> {
    write!(f, "name {}. age: {}, ", self.name, self.age)
  }
}

pub fn hello_word {
    println!("Hello world! I'm a rustocean");
    print!("abcdefg");
  
    eprint!("换行");
    println!("{} days", 120);
    println!("{0}  {1}  {1}  {0}", 0, 1);
    println!("My name is {name}.", name = "kelvin");
    println!("{} {:b}", 1, 10);

    let pi = 3.141592692;
    println!("Pi is roughly {:.2}", pi);

    let kelvin = Person::new(String::from("kelvin"), 29);
    println!("{:#?}", kelvin);
    println!("{}", kelvin);
    
}