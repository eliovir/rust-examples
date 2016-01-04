/**
 * 4.2 Pattern matching
 * http://doc.rust-lang.org/tutorial.html#pattern-matching
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern crate rand;

use std::f64;
use rand::Rng;

struct Point { x: f64, y: f64 }

fn angle(vector: (f64, f64)) -> f64 {
	let pi = f64::consts::PI;
	/*
	 * A powerful application of pattern matching is destructuring:
	 * matching in order to bind names to the contents of data types.
	 */
	match vector {
		(0.0, y) if y < 0.0 => 1.5 * pi,
		(0.0, _) => 0.5 * pi,
		(x, y) => (y / x).atan()
	}
}
fn main() {
	let my_number: u32 = rand::thread_rng().gen_range(0, 10);
	println!("my_number = {}", my_number);

	/*
	 * Rust's match construct is a generalized, cleaned-up version of C's
	 * switch construct. You provide it with a value and a number of arms,
	 * each labelled with a pattern, and the code compares the value
	 * against each pattern in order until one matches. The matching
	 * pattern executes its corresponding arm.
	 */
	match my_number {
		0     => println!("zero"),
		1 | 2 => println!("one or two"),
		3...10 => println!("three to ten"),
		_     => println!("something else")
	}
	/*
	 * match constructs must be exhaustive: they must have an arm covering
	 * every possible case. For example, the typechecker would reject the
	 * previous example if the arm with the wildcard pattern was omitted.
	 */
	match my_number {
		0 => { println!("zero") }
		_ => { println!("something else") }
	}

	let age: u32 = rand::thread_rng().gen_range(0, 100);
	println!("age = {}", age);
	match age {
		a @ 0...20 => println!("{} years old", a),
		_ => println!("older than 21")
	}

	let vector = (1f64,1f64);
	println!("angle({:?}) == {:?}", vector, angle(vector));

	/*
	 * To destructure a struct, use `..`.
	 * http://doc.rust-lang.org/tutorial.html#structs
	 */
	let mypoint = Point { x: 0.0, y: 0.0 };
	match mypoint {
		Point { x, .. } => println!("{}", x),
	}
	match mypoint {
		Point { y, .. } => println!("y={}", y),
	}
}
