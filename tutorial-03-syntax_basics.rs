/**
 * 3 Syntax basics
 * http://static.rust-lang.org/doc/0.7/tutorial.html#syntax-basics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn main() {
	/*
	 * The let keyword introduces a local variable.
	 * Variables are immutable by default.
	 * To introduce a local variable that you can re-assign later, use let mut instead.
	 */
	let hi = "hi";
	let mut count = 0;

	while count < 10 {
		    println(fmt!("%s, count: %?", hi, count));
		        count += 1;
	}
}
