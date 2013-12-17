#[feature(struct_variant)];
/**
 * 16 Methods
 * http://static.rust-lang.org/doc/master/tutorial.html#methods
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::f64::consts::PI;

struct Point {
	x: f64,
	y: f64
}
enum Shape {
	Circle { center: Point, radius: f64 },
	Rectangle { top_left: Point, bottom_right: Point }
}
impl Shape {
	fn draw(&self) {
		match *self {
			Circle{center: p, radius: f} => draw_circle(p, f),
			Rectangle{top_left: p1, bottom_right: p2} => draw_rectangle(p1, p2)
		}
	}
	pub fn new_circle(area: f64) -> Shape {
		let center = Point{x: 0.0, y: 0.0};
		let radius = (area / PI).sqrt();
		Circle{center: center, radius: radius}
	}
}
fn draw_circle(p: Point, f: f64) {
	println!("draw_circle({:?}, {:f})", p, f);
}
fn draw_rectangle(p1: Point, p2: Point) {
	println!("draw_rectangle({:?}, {:?})", p1, p2);
}
fn main() {
	let c = Circle{center: Point { x: 1.0, y: 2.0 }, radius: 3.0};
	c.draw();

	let r = Rectangle{top_left: Point{x: 1.0, y: 2.0}, bottom_right: Point{x: 2.0, y: 3.0}};
	r.draw();

	let c2 = Shape::new_circle(42.5);
	println!("c2={:?}", c2);
	c2.draw();
}
