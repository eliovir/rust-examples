/**
 * https://github.com/mozilla/rust/wiki/Doc-unit-testing
 * rustc unittest.rs --test -o unittest
 * ./unittest --test
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
#[cfg(test)]
mod tests {
	#[test]
	fn testFail() {
		assert!(1 == 2, "This test must fail!");
	}
	#[test]
	fn testSuccess() {
		assert!(1 == 1);
	}
}
fn main() {
	println("This program must be build and run with --test");
}
