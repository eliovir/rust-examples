/**
 * 14 Closure
 * http://static.rust-lang.org/doc/master/tutorial.html#closures
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

fn apply(i: int, f: &fn(int)->int) -> int {
	f(i)
}

fn call_closure_with_ten(f: &fn(int)) {
	f(10);
}

fn main() {
	/*
	 * Simple call of closure
	 */
	let res = apply(4, |x| { x*x});
	println!("apply(4, |x| x*x) => {:d}", res);

	/*
	 * Call with captured variable
	 */
	let captured_var = 20;
	let closure = |arg| println!("captured_var={}, arg={}", captured_var, arg);
	call_closure_with_ten(closure);

	/*
	 * Use of .map() on a vector
	 */
	let mut max = 0i;
	[1, 2, 3].map(|x| if *x > max { max = *x });
	println!("max={}", max);
}
