extern crate debug;
use std::rc::Rc;

/**
 * Snippets from the Dave Herman's presentation (2013-01-06)
 * http://www.infoq.com/presentations/Rust
 *
 * Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
 *
 * 12. Dereferencing pointers
 * http://doc.rust-lang.org/tutorial.html#dereferencing-pointers
 *
 * The deprecation of @, its alternatives
 * https://github.com/rust-lang/rust/wiki/Doc-detailed-release-notes
 */
struct Point {
	x: f64,
	y: f64
}

struct Rectangle (Point, Point);

impl Rectangle {
	fn area(&self) -> f64 {
		match *self {
			Rectangle(top_left, bottom_right) => (bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
		}
	}
}

fn print_point(p: &Point) {
	match *p {
		Point {x, y} => println!("({:f}, {:f})", x, y)
	}
}

fn main() {
	/*
	 * Snippets from the Dave Herman's presentation (2013-01-06)
	 * http://www.infoq.com/presentations/Rust
	 *
	 * Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
	 */
	// Borrowed pointer to T
	// &T, in C++ : T&
	let p = Point{x:1.1, y:1.2};
	print_point(&p);
	// Owned pointer to T
	// box T, in C++ : unique_ptr<T>
	let p3 = box Point{x:3.1, y:3.2};
	print_point(&*p3);

	/*
	 * 13. Dereferencing pointers
	 * http://doc.rust-lang.org/tutorial.html#dereferencing-pointers
	 */
	let owned = box 20u;
	let borrowed = &30u;

	let sum = *owned + *borrowed;
	println!("{} + {} = {}", *owned, *borrowed, sum);

	/*
	 * Dereferenced mutable pointers may appear on the left hand side of assignments.
	 * Such an assignment modifies the value that the pointer points to.
	 */
	let mut owned = box 20u;

	let mut value = 30u;
	let borrowed = &mut value;

	*owned = *borrowed + 100;
	let sum = *owned + *borrowed;
	println!("{} + {} = {}", *owned, *borrowed, sum);

	/*
	 * Pointers have high operator precedence, but lower precedence than the dot
	 * operator used for field and method access. This precedence order can
	 * sometimes make code awkward and parenthesis-filled.
	 *
	 * To combat this ugliness the dot operator applies automatic pointer
	 * dereferencing to the receiver (the value on the left-hand side of the dot),
	 * so in most cases, explicitly dereferencing the receiver is not necessary.
	 */
	let bottom = box Point { x: 10.0, y: 120.0 };
	let top = box Point { x: bottom.x + 100.0, y: bottom.y - 100.0 };
	let rect = &Rectangle(*top, *bottom);
	let area = rect.area();
	println!("Area of rectangle {:?}: {:f}", rect, area);

	/*
	 * http://doc.rust-lang.org/std/rc/struct.Rc.html#method.deref
	 * Note that to get the value inside a Rc box you first call
	 * the deref() method, with return &T, then dereference that.
	 *
	 * http://doc.rust-lang.org/std/gc/index.html
	 */
	let rc1 = Rc::new(1u);
	let rc2 = rc1.clone();
	println!("{:u}", *(rc1.deref()) + *(rc2.deref()));
}
