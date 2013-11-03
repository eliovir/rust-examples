#[feature(struct_variant)];
/**
 * 5.2 Enums
 * http://static.rust-lang.org/doc/master/tutorial.html#enums
 * http://static.rust-lang.org/doc/master/rust.html#compiler-features
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::f64;

struct Point {
	x: f64,
	y: f64
}

enum Shape {
	Circle { center: Point, radius: f64 },
	Rectangle { top_left: Point, bottom_right: Point }
}

/**
 * Compute area of a Shape
 */
fn area(sh: Shape) -> f64 {
	match sh {
		Circle { radius: radius, _ } => f64::consts::PI * radius * radius,
		Rectangle { top_left: top_left, bottom_right: bottom_right } => {
			(bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
		}
	}
}

/**
 * Main program
 */
fn main() {
	let top_left = Point { x: 0.0, y: 1.0 };
	let bottom_right = Point { x: 1.0, y: 0.0 };
	let my_circle = Circle { center: top_left, radius: 1.0};
	let my_rectangle = Rectangle { top_left: top_left, bottom_right: bottom_right};
	let circle_area: f64 = area(my_circle);
	let rectangle_area: f64 = area(my_rectangle);
	println!("circle area: {:f}", circle_area);
	println!("rectangle area: {:f}", rectangle_area);
}

