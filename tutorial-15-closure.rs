/**
 * 15 Closure
 * http://doc.rust-lang.org/tutorial.html#closures
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern crate debug;

fn apply(i: int, f: |int|->int) -> int {
	f(i)
}

fn call_closure_with_ten(b: |int|) {
	b(10);
}

// A function that takes a procedure as argument:
fn call_it(op: proc(v: int)) {
	op(10)
}

// |v: &int| is stack closure, not procedure: procedure is allocated at heap, and you can call it only once.
fn each(v: &[int], op: |v: &int|) {
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
	println!("apply(4, |x| x*x) => {:d}", res);

	/*
	 * Call with captured variable
	 */
	let captured_var = 20i;
	let closure = |arg| println!("captured_var={}, arg={}", captured_var, arg);
	call_closure_with_ten(closure);

	/*
	 * Use of closure as map on a vector
	 */
	let mut max = 0i;
	{
		let find_max = |x: int| if x > max { max = x };
		for x in [1, 2, 3].iter() {
			find_max(*x);
		}
	}
	println!("max={}", max);

	/*
	 * Use of .map() on a vector
	 */
	[1, 2, 3].iter().map(|x| if *x > max { max = *x });
	println!("max={}", max);

	/*
	 * As a caller, if we use a closure to provide the final operator argument, we can write it in a way that has a pleasant, block-like structure.
	 */
	call_it(proc(n) {
		println!("{:?}", n);
	});
	each([1, 2, 3], |n: &int| {
		println!("{:?}", n);
	});
}
