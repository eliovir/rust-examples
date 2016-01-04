/**
 * The Rust Reference Manual
 * 7.2.14 Lambda expressions
 *
 * The Rust Programming Language
 * 5.23 Closures
 * https://doc.rust-lang.org/stable/book/closures.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn ten_times<F>(f: F) where F : Fn(i32) {
	let mut i = 0;
	while i < 10 {
		f(i);
		i += 1;
	}
}

fn main() {
	ten_times(|j| println!("hello, {}", j));
}
