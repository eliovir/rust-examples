/**
 * 4.3 Loops
 * http://static.rust-lang.org/doc/master/tutorial.html#loops
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
struct Pair {
	x: int,
	y: int
}
fn main() {
	/*
	 * An example of a for loop over the contents of a vector:
	 */
	let v: &[uint] = &[4, 2, 1];
	for i in v.iter() {
		println!("{:u} is an integer!", *i);
	}
		
	/*
	 * A standard library function that applies a closure to every number between 0 and 10.
	 */
	for i in range(0u, 10) {
		println!("{:u} is an integer!", i);
	}

	/*
	 * is inlined by the compiler as:
	 */
	let mut j = 0u;
	while j < 10 {
		println!("{:u} is an integer!", j);
		j += 1;
	}

	/*
	 * Using loop.
	 */
	let mut k = 0u;
	loop {
		k += 1;
		if k == 10 {
			break;
		}
		println!("{:u} is an integer!", k);
	}

	/*
	 * Destructuring a struct in for loops:
	 */
	let pairs = ~[Pair {x: 10, y: 20}, Pair {x: 30, y: 0}]; 

	for &Pair {x, y} in pairs.iter() {
		assert_eq!(x + y, 30);
	}
}
