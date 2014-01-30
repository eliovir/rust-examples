/**
 * http://static.rust-lang.org/doc/master/std/vec/index.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::vec;

/**
 * ~[~str] is an owned pointer, allocated on the send heap, can be sent accross tasks.
 */
fn fillStrings() -> ~[~str] {
	let mut strings: ~[~str] = ~[];
	strings.push(~"hello");
	strings.push(~"world");
	strings
}

/**
 * Use of vec::from_elem to create a dynamic two dimensional array.
 */
fn make2dArray(dim1: uint, dim2: uint, default: int) -> ~[~[int]] {
	vec::from_elem(dim1, vec::from_elem(dim2, default))
}

fn main() {
	/*
	 * Simple use
	 * .iter() returns an iteration value for a vector or a vector slice.
	 *  The iterator yields borrowed pointers to the vector's elements,
	 *  so if the element type of the vector is int, the element type of the iterator is &int.
	 */
	let numbers = [0, 1, 2];
	for &x in numbers.iter() {
		println!("element in numbers: {}", x);
	}

	/*
	 * .slice(a, b) returns an immutable "view" into a vector or a vector slice from the interval (a, b)
	 * .push() adds an item at the end of an OwnedVector
	 */
	let mut numbers2 = ~[];
	for &x in numbers.slice(1, 3).iter() {
		numbers2.push(x);
	}
	for &x in numbers2.iter() {
		println!("element in numbers2: {}", x);
	}

	/*
	 * You can destructure vectors:
	 *
	 * `unreachable!()` expands to `fail!("internal error: entered unreachable code")`.
	 */
	let v = ~[1, 2, 3, 4, 5];
	match v {
		[]                       => println!("empty"),
		[elem]                   => println!("{}", elem),   // => 1
		[first, second, ..rest]  => println!("first={}, second={}, rest={:?}", first, second, rest)  // => &[3, 4, 5]
	}
	match v {
		[first, ..] => assert_eq!(first, 1),
		[.., last]  => assert_eq!(last, 3),
		_           => unreachable!()
	}
	match v {
		[first, .. middle, last] => println!("first={:?}, middle={:?}, last={:?}", first, middle, last),
		_                        => unreachable!()
	}

	/*
	 * Create a vector in a function.
	 */
	let mut strings = fillStrings();
	println!("strings[1] = {}", strings[1]);
	strings[1] = ~"me";
	println!("strings[1] = {}", strings[1]);

	/*
	 * Create a two dimensional vector.
	 */
	let mut array2d = [[0u8, ..4], ..4];
	println!("array2d[0][0] = {}", array2d[0][0]);
	array2d[0][1] = 1;
	println!("array2d[0][1] = {}", array2d[0][1]);

	/*
	 * Create a two dimensional dynamic vector.
	 */
	let mut anotherArray2d = make2dArray(2, 3, -1);
	println!("anotherArray2d[0][0] = {}", anotherArray2d[0][0]);
	anotherArray2d[0][1] = 1;
	println!("anotherArray2d[0][1] = {}", anotherArray2d[0][1]);
}

