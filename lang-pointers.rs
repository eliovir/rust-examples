/**
 * Snippets from the Dave Herman's presentation (2013-01-06)
 * http://www.infoq.com/presentations/Rust
 *
 * Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
 */
struct Point {
	x: float,
	y: float
}
fn print_point(p: &Point) {
	match *p {
		Point {x, y} => println(fmt!("(%f, %f)", x, y))
	}
}
fn main() {
	// Borrowed pointer to T
	// &T, in C++ : T&
	let p = Point{x:1.1, y:1.2};
	print_point(&p);
	// Managed pointer to T
	// @T, in C++ : shared_ptr<T>
	let p2 = @Point{x:2.1, y:2.2};
	print_point(p2);
	// Owned pointer to T
	// ~T, in C++ : unique_ptr<T>
	let p3 = ~Point{x:3.1, y:3.2};
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
}
