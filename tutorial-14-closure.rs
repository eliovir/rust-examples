/**
 * 14 Closure
 * http://static.rust-lang.org/doc/0.8/tutorial.html#closures
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn call_closure_with_ten(b: &fn(int)) { b(10); }

fn main() {
	let captured_var = 20;
	let closure = |arg| println(fmt!("captured_var=%d, arg=%d", captured_var, arg));
	call_closure_with_ten(closure);
	/*
	 */
	let mut max = 0i;
	[1, 2, 3].map(|x| if *x > max { max = *x });
	println(fmt!("max=%d", max));
}
