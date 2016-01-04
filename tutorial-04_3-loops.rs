/**
 * 4.3 Loops
 * http://doc.rust-lang.org/tutorial.html#loops
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
struct Pair {
	x: i32,
	y: i32
}
fn main() {
	/*
	 * An example of a for loop over the contents of a vector:
	 */
	let v: &[u32] = &[4, 2, 1];
	for i in v.iter() {
		println!("{} is an integer!", *i);
	}

	/*
	 * A standard library function that applies a closure to every number between 0 and 10.
	 */
	for i in 0..10 {
		println!("{} is an integer!", i);
	}

	/*
	 * is inlined by the compiler as:
	 */
	let mut j = 0usize;
	while j < 10 {
		println!("{} is an integer!", j);
		j += 1;
	}

	/*
	 * Using loop.
	 */
	let mut k = 0usize;
	loop {
		k += 1;
		if k == 10 {
			break;
		}
		println!("{} is an integer!", k);
	}

	/*
	 * More generally, a for loop works with anything implementing the `Iterator` trait.
	 * Data structures can provide one or more methods that return iterators over their contents.
	 * For example, strings support iteration over their contents in various ways:
	 */
	let s = "Hello";
	for c in s.chars() {
    		println!("{}", c);
	}

	/*
	 * Destructuring a struct in for loops:
	 */
	let pairs = [Pair {x: 10, y: 20}, Pair {x: 30, y: 0}];

	for &Pair {x, y} in pairs.iter() {
		assert_eq!(x + y, 30);
	}
}
