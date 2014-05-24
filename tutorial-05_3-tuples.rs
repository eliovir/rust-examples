/**
 * 5.3 Tuples
 * http://doc.rust-lang.org/tutorial.html#tuples
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn main() {
	// A Tuple is an immutable fixed-size collection of values. 

	let tuple0 = ();
	println!("{:?}", tuple0);

	let mytup: (int, int, f64) = (10, 20, 30.0);
	match mytup {
		(a, b, c) => println!("{}", a + b + (c as int))
	}

	let tuple1 = (5i, 6i);
	let (first, _) = tuple1;
	println!("{}", first);
}
