extern crate num;


use std::f64;
use std::iter::Map;
use std::ops::Range;


fn main() {
    println!("Hello, world!");
    let mut doors = [false; 100];
    solve(&mut doors);

    for(idx, door) in doors.iter().enumerate() {
        println!("door {} open: {}", idx + 1, door);
    }

    let doors = calculate_doors();
    for (i, x) in doors.enumerate() {
        println!("Door {} is {:?}", i + 1, x);
    }
}

fn solve(doors: &mut [bool]) {
    for pass in 1..101 {
        let mut p = pass;
        while p <= 100 {
            doors[p-1] = !doors[p -1];
            p += pass;
        }
    }
}


type DoorIter = Map<Range<u32>, fn(u32) -> DoorState>;

#[derive(Debug, PartialEq)]
enum DoorState {
  Open,
  Closed,
}

fn calculate_doors() -> DoorIter {
  fn door_status(door_number: u32) -> DoorState {
    let x = f64::from(door_number).sqrt();
    if (x - x.round()).abs() < f64::EPSILON {
      DoorState::Open
    } else{
      DoorState::Closed
    }
  }

  (1u32..101).map(door_status as fn(u32) -> DoorState)
}



#[test]
fn solution() {
    let mut doors = [false; 100];
    solve(&mut doors);

    for i in 1..11 {
        assert!(doors[i * i - 1]);
    }
}

#[test]
fn solution2() {
    let doors = calculate_doors().collect::<Vec<DoorState>>();
    for i in 1..11 {
        assert_eq!(doors[i * i -1], DoorState::Open);
    }
}