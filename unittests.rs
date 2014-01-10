/**
 * https://github.com/mozilla/rust/wiki/Doc-unit-testing
 * rustc unittest.rs --test -o unittest
 * ./unittest --test
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
#[cfg(test)]
mod tests {
	/* This test is commented to avoid false negative tests.
	#[test]
	fn testFail() {
		assert!(1 == 2, "This test must fail!");
	}
	*/
	#[test]
	fn testFloat() {
		let expected = 1f64;
		let mut actual = 1f64;
		let precision = 0.1f64;
		actual = actual + precision / 2f64;
		assert!((expected - actual).abs() < precision);
	}
	#[test]
	fn testSuccess() {
		assert!(1 == 1);
		assert_eq!(1, 1);
	}
}
/*
fn main() {
	println("This program must be build and run with --test");
}
*/
