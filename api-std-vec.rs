/**
 * http://doc.rust-lang.org/std/vec/
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

/**
 * ~[~str] is an owned pointer, allocated on the send heap, can be sent accross tasks.
 * Vec<T> is what ~[T] was before it was removed.
 */
fn fill_strings() -> Vec<String> {
	let mut strings = vec!("hello".to_string());
	strings.push("world".to_string());
	strings
}

/**
 * Use of Vec::with_capacity to create a dynamic two dimensional array.
 */
fn make_2d_array(dim1: usize, dim2: usize, default: i32) -> Vec<Vec<i32>> {
	let mut vec: Vec<Vec<i32>> = Vec::with_capacity(dim1);
	for _i in 0..dim1 {
		let mut vec2 : Vec<i32> = Vec::with_capacity(dim2);
		for _j in 0..dim2 {
			vec2.push(default);
		}
		vec.push(vec2);
	}
	vec
}

fn main() {
	/*
	 * Simple use
	 * .iter() returns an iteration value for a vector or a vector slice.
	 *  The iterator yields borrowed pointers to the vector's elements,
	 *  so if the element type of the vector is i32, the element type of the iterator is &i32.
	 */
	let numbers = [0i32, 1, 2];
	for &x in numbers.iter() {
		println!("element in numbers: {}", x);
	}
	/*
	 * No need to use iter().
	 */
	for x in &numbers {
		println!("element in numbers: {}", x);
	}

	/*
	 * The slice [a..b] returns an immutable "view" into a vector or a vector slice from the interval (a, b)
	 * .push() adds an item at the end of an OwnedVector
	 */
	let mut numbers2 = Vec::new();
	for &x in numbers[1..3].iter() {
		numbers2.push(x);
	}
	for &x in numbers2.iter() {
		println!("element in numbers2: {}", x);
	}

	/*
	 * Create a vector in a function.
	 */
	let mut strings = fill_strings();
	println!("strings[1] = {}", strings[1]);
	strings[1] = "me".to_string();
	println!("strings[1] = {}", strings[1]);

	/*
	 * Create a two dimensional dynamic vector.
	 */
	let mut array2d = vec![vec![0; 4]; 4];
	println!("array2d[0][0] = {}", array2d[0][0]);
	array2d[0][1] = 1;
	println!("array2d[0][1] = {}", array2d[0][1]);

	/*
	 * Create a two dimensional dynamic vector.
	 */
	let mut another_array2d = make_2d_array(2, 3, -1);
	println!("another_array2d[0][0] = {}", another_array2d[0][0]);
	another_array2d[0][1] = 1;
	println!("another_array2d[0][1] = {}", another_array2d.get(0).unwrap().get(1).unwrap());
}

