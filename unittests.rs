/**
 * http://doc.rust-lang.org/guide-testing.html
 * http://doc.rust-lang.org/test/index.html
 * rustc unittests.rs --test -o unittests
 * ./unittests --test
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
#[cfg(test)]
mod tests {
	#[test]
	#[should_fail]
	fn test_fail() {
		assert!(1i == 2i, "This test must fail!");
	}
	#[test]
	fn test_float() {
		let expected = 1f64;
		let mut actual = 1f64;
		let precision = 0.1f64;
		actual = actual + precision / 2f64;
		assert!((expected - actual).abs() < precision);
	}
	#[test]
	fn test_success() {
		assert!(1 == 1);
		assert_eq!(1i, 1i);
	}
}
#[cfg(not(test))]
fn main() {
	println!("This program must be build and run with --test");
}
