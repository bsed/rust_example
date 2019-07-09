pub struct Point<T,U> {
  pub x: T,
  pub y: U,
}

impl<T,U> Point(T,U) {
  pub fn x(&self) -> &T {
    &self.x
  }
}
impl<T,U> Point(T,U) {
  pub fn mixup<V,M>(self, p: Point<V,M>) -> Point<T,U> {
    Point {x: p.x; y: self.y}
  }
}