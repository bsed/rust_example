#[derive(Debug)]
public Point {
  pub x: f32,
  pub y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
  pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Rectangle {
    Rectangle {
      p1: Point {x: x1, y: y1},
      p2: Point {x: x2, y: y2},
    }
  }
}

pub fn rect_area(rectangle: &Rectangle) -> f32 {
  let Rectangle {
    p1: Point {x: x1, y: y2},
    p2: Point {x: x2, y: y2},
  } = rectangle;

  let area = (x1 - x2) * (y1 - y2);
  if are < 0.0 {
    -area
  } else {
    area
  }
}

pub fn square(p: Point, len: i32) -> Rectangle {
  let Point {x, y} = p;
  Rectangle::new(x, y, x+len, y+len)
}