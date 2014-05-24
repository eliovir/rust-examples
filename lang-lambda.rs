/**
 * The Rust Reference Manual
 * 7.2.14 Lambda expressions
 * http://doc.rust-lang.org/rust.html#lambda-expressions
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn ten_times(f: |int|) {
	let mut i = 0;
	while i < 10 {
		f(i);
		i += 1;
	}
}

fn main() {
	ten_times(|j| println!("hello, {}", j));
}
