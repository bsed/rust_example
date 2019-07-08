use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
//use std::process::{Command, Stdio};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[link(name = "m")]
extern "C" {
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

static NTHREADS: i32 = 10;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    println!("{:?}", ids);

    let z = Complex { re: -1., im: 0. };

    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    println!("cos({:?}) = {:?}", z, cos(z));

    let path = Path::new(".");
    //let display = path.display();
    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }

    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }

    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let path1 = Path::new("hello.txt");
    let display1 = path1.display();
    let mut file1 = match File::open(&path1) {
        Err(why) => panic!("couldn't open {}: {}", display1, why.description()),
        Ok(file1) => file1,
    };

    let mut s1 = String::new();
    match file1.read_to_string(&mut s1) {
        Err(why) => panic!("couldn't read {}: {} ", display1, why.description()),
        Ok(_) => print!("{} contains:\n{}", display1, s1),
    }

    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }

    // let process2 = match Command::new("wc")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()
    // {
    //     Err(why) => panic!("couldn't spawn wc: {}", why.description()),
    //     Ok(process2) => process2,
    // };
    // match process2.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
    //     Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
    //     Ok(_) => println!("sent pangram to wc"),
    // }

    // let mut s = String::new();
    // match process2.stdout.unwrap().read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read wc stdout: {}",
    //                        why.description()),
    //     Ok(_) => print!("wc responded with:\n{}", s),
    // }

    let mut child3 = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child3.wait().unwrap();
    println!("reached end of main");
}
