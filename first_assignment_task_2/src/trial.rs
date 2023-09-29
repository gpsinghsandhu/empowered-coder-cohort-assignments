// Define a simple struct
struct Point {
  x: f64,
    y: f64,
}

impl Point {
    // Method 1: Move the point by a given offset
    fn translate(& mut self, dx: f64, dy: f64) {
    self.x += dx;
    self.y += dy;
  }

    // Method 2: Scale the point by a given factor
    fn scale(& mut self, factor: f64) {
    self.x *= factor;
    self.y *= factor;
  }
}

fn main() {
  let mut p = Point { x: 1.0, y: 2.0 };

  // Call the translate method to move the point
  p.translate(2.0, 3.0);
  println!("After translation: x = {}, y = {}", p.x, p.y);

  // Call the scale method to scale the point
  p.scale(0.5);
  println!("After scaling: x = {}, y = {}", p.x, p.y);
}
