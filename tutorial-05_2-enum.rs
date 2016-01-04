#![allow(dead_code)]
/**
 * 5.2 Enums
 * http://doc.rust-lang.org/tutorial.html#enums
 * http://doc.rust-lang.org/rust.html#compiler-features
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct Point {
	x: f64,
	y: f64
}

/*
 * Enum variant
 */
enum Shape {
	Circle(Point, f64),
	Rectangle(Point, Point)
}

fn area(sh: Shape) -> f64 {
    match sh {
        Shape::Circle(_, size) => PI * size * size,
        Shape::Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y - y2)
    }
}

/*
 * Enum of structs
 */
enum StructShape {
	StructCircle { center: Point, radius: f64 },
	StructRectangle { top_left: Point, bottom_right: Point }
}

/**
 * Compute area of a Shape
 */
fn struct_area(sh: StructShape) -> f64 {
	match sh {
		StructShape::StructCircle { radius, .. } => PI * radius * radius,
		StructShape::StructRectangle { top_left, bottom_right } => {
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

	/*
	 * Enum variant
	 */
	let my_circle1 = Shape::Circle(top_left, 1.0);
	let my_rectangle1 = Shape::Rectangle(top_left, bottom_right);
	let circle_area1: f64 = area(my_circle1);
	let rectangle_area1: f64 = area(my_rectangle1);
	println!("variant: circle area: {}", circle_area1);
	println!("variant: rectangle area: {}", rectangle_area1);

	/*
	 * Enum of structs
	 */
	let my_circle = StructShape::StructCircle { center: top_left, radius: 1.0};
	let my_rectangle = StructShape::StructRectangle { top_left: top_left, bottom_right: bottom_right};
	let circle_area: f64 = struct_area(my_circle);
	let rectangle_area: f64 = struct_area(my_rectangle);
	println!("struct: circle area: {}", circle_area);
	println!("struct: rectangle area: {}", rectangle_area);
}

