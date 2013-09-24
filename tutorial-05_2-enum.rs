/**
 * 5.2 Enums
 * http://static.rust-lang.org/doc/0.7/tutorial.html#enums
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::float;

struct Point {
	x: float,
	y: float
}

enum Shape {
	Circle { center: Point, radius: float },
	Rectangle { top_left: Point, bottom_right: Point }
}

/**
 * Compute area of a Shape
 */
fn area(sh: Shape) -> float {
	match sh {
		Circle { radius: radius, _ } => float::consts::pi * radius * radius,
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
	let circle_area: float = area(my_circle);
	let rectangle_area: float = area(my_rectangle);
	println(fmt!("circle area: %f", circle_area));
	println(fmt!("rectangle area: %f", rectangle_area));
}

