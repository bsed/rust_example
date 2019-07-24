extern crate num;

use std::f64;
use std::iter::Map;
use std::ops::Range;

type DoorIter = Map<Range<u32>, fn(u32)> -> DoorState>;

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

fn main() {
  
}