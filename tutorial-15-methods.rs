/**
 * 15 Methods
 * http://static.rust-lang.org/doc/0.8/tutorial.html#methods
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::float::consts::pi;

struct Point {
	x: float,
	y: float
}
enum Shape {
	Circle { center: Point, radius: float },
	Rectangle { top_left: Point, bottom_right: Point }
}
impl Shape {
	fn draw(&self) {
		match *self {
			Circle{center: p, radius: f} => draw_circle(p, f),
			Rectangle{top_left: p1, bottom_right: p2} => draw_rectangle(p1, p2)
		}
	}
	pub fn new_circle(area: float) -> Shape {
		let center = Point{x: 0f, y: 0f};
		let radius = (area / pi).sqrt();
		Circle{center: center, radius: radius}
	}
}
fn draw_circle(p: Point, f: float) {
	println(fmt!("draw_circle(%?, %?)", p, f));
}
fn draw_rectangle(p1: Point, p2: Point) {
	println(fmt!("draw_rectangle(%?, %?)", p1, p2));
}
fn main() {
	let c = Circle{center: Point { x: 1f, y: 2f }, radius: 3f};
	c.draw();

	let r = Rectangle{top_left: Point{x: 1f, y: 2f}, bottom_right: Point{x: 2f, y: 3f}};
	r.draw();

	let c2 = Shape::new_circle(42.5);
	println(fmt!("c2=%?", c2));
	c2.draw();
}
