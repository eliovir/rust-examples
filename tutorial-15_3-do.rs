/**
 * 15.3 Do syntax
 * http://static.rust-lang.org/doc/master/tutorial.html#do-syntax
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
// The do expression makes it easier to call functions that take procedures as arguments.

// A function that takes a procedure as argument:
fn call_it(op: proc(v: int)) {
	    op(10)
}

fn main() {
	// As a caller, if we use a closure to provide the final operator argument, we can write it in a way that has a pleasant, block-like structure.
	call_it(proc(n) {
		    println(n.to_str());
	});

	// This is such a useful pattern that Rust has a special form of function call for these functions.
	do call_it() |n| {
		    println(n.to_str());
	}

}
