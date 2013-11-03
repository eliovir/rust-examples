/**
 * 4.3 Loops
 * http://static.rust-lang.org/doc/master/tutorial.html#loops
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn main() {
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
}
