#![feature(managed_boxes)]
use std::rc::Rc;
use std::gc::Gc;

/**
 * Snippets from the Dave Herman's presentation (2013-01-06)
 * http://www.infoq.com/presentations/Rust
 *
 * Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
 * 
 * 13. Dereferencing pointers
 * http://static.rust-lang.org/doc/master/tutorial.html#dereferencing-pointers
 *
 * The deprecation of @, its alternatives
 * https://github.com/mozilla/rust/wiki/Doc-detailed-release-notes
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
	// Managed pointer to T
	// @T, in C++ : shared_ptr<T>
	let p2 = @Point{x:2.1, y:2.2};
	print_point(p2);
	// Owned pointer to T
	// box T, in C++ : unique_ptr<T>
	let p3 = box Point{x:3.1, y:3.2};
	print_point(p3);
	// Unsafe pointer to T
	// *T
	/*
	let p4; // uninitialized
	print_point(p4); // error
	
	let p5 = ~Point {x:5.1, y:5.2};
	box.topLeft = move p5; // deinitialized
	print_point(p); // error
	*/

	/*
	 * 13. Dereferencing pointers
	 * http://static.rust-lang.org/doc/master/tutorial.html#dereferencing-pointers
	 */
	let managed = @10;
	let owned = box 20;
	let borrowed = &30;
	
	let sum = *managed + *owned + *borrowed;
	println!("{} + {} + {} = {}", *managed, *owned, *borrowed, sum);

	/*
	 * Dereferenced mutable pointers may appear on the left hand side of assignments.
	 * Such an assignment modifies the value that the pointer points to.
	 */
	let managed = @10;
	let mut owned = box 20;

	let mut value = 30;
	let borrowed = &mut value;

	*owned = *borrowed + 100;
	*borrowed = *managed + 1000;
	let sum = *managed + *owned + *borrowed;
	println!("{} + {} + {} = {}", *managed, *owned, *borrowed, sum);

	/*
	 * Pointers have high operator precedence, but lower precedence than the dot
	 * operator used for field and method access. This precedence order can
	 * sometimes make code awkward and parenthesis-filled.
	 * 
	 * To combat this ugliness the dot operator applies automatic pointer
	 * dereferencing to the receiver (the value on the left-hand side of the dot),
	 * so in most cases, explicitly dereferencing the receiver is not necessary.
	 */
	let bottom = @Point { x: 10.0, y: 120.0 };
	let top = box Point { x: bottom.x + 100.0, y: bottom.y - 100.0 };
	let rect = &Rectangle(*top, *bottom);
	let area = rect.area();
	println!("Area of rectangle {:?}: {:f}", rect, area);

	/*
	 * http://static.rust-lang.org/doc/master/std/rc/struct.Rc.html#method.deref
	 * Note that to get the value inside a Rc box you first call
	 * the deref() method, with return &T, then dereference that.
	 *
	 * http://static.rust-lang.org/doc/master/std/gc/struct.Gc.html#method.borrow
	 * Note that to get the value inside a Gc box you first call
	 * the borrow() method, which returns &T, then dereference that. 
	 */
	let rc1 = Rc::new(1u);
	let rc2 = rc1.clone();
	println!("{:u}", *(rc1.deref()) + *(rc2.deref()));

	let gc1 = Gc::new(1u);
	let gc2 = gc1.clone();
	println!("{:u}", *(gc1.borrow()) + *(gc2.borrow()));
}
