/**
 * 15 Closure
 * http://doc.rust-lang.org/tutorial.html#closures
 * 5.23 Closures
 * https://doc.rust-lang.org/stable/book/closures.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

fn apply<F>(i: i32, f: F) -> i32
    where F : Fn(i32) -> i32 {
	f(i)
}

fn call_closure_with_ten<F>(b: F)
    where F : Fn(i32) {
	b(10);
}

//A function pointer is kind of like a closure that has no environment. As such, you can pass a
//function pointer to any function expecting a closure argument, and it will work:
fn call_it<F>(op: F)
    where F : Fn(i32) {
	op(10)
}

fn each<F>(v: &[i32], op: F)
    where F : Fn(&i32) {
	let mut n = 0;
	while n < v.len() {
		op(&v[n]);
		n += 1;
	}
}

fn main() {
	/*
	 * Simple call of closure
	 */
	let res = apply(4, |x| { x*x});
	println!("apply(4, |x| x*x) => {}", res);

	/*
	 * Call with captured variable
	 */
	let captured_var = 20;
	let closure = |arg| println!("captured_var={}, arg={}", captured_var, arg);
	call_closure_with_ten(closure);

	/*
	 * Use of closure as map on a vector
	 */
	let mut max = 0;
	{
		let mut find_max = |x: i32| if x > max { max = x };
		for x in [1, 2, 3].iter() {
			find_max(*x);
		}
	}
	println!("max={}", max);

	/*
	 * As a caller, if we use a closure to provide the final operator argument, we can write it in a way that has a pleasant, block-like structure.
	 */
	call_it(|n| {
		println!("{:?}", n);
	});
	each(&[1i32, 2, 3], |n: &i32| {
		println!("{:?}", n);
	});
}
