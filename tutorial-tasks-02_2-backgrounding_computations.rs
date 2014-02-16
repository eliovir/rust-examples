/**
 * Rust Tasks and Communication Tutorial - 2.2 Backgrounding computations: Futures
 * http://static.rust-lang.org/doc/master/guide-tasks.html#backgrounding-computations-futures
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern crate extra;
extern crate fibonacci;
#[cfg(not(test))]
use std::vec;

fn partial_sum(start: uint) -> f64 {
	let mut local_sum = 0f64;
	for num in range(start*100000, (start+1)*100000) {
		local_sum += (num as f64 + 1.0).powf(&-2.0);
	}
	local_sum
}
#[test]
fn test_partial_sum_5() {
	let param = 5u;
	let expected = 0.00000033f64;
	let actual = partial_sum(param);
	let precision = 1e-06f64;
	assert!((expected - actual).abs() < precision);
}
#[cfg(not(test))]
fn main() {
	let n = 40;
	println("Setting spawn");
	/*
	 * Note that the future needs to be mutable so that it can save the result for next time get is called.
	 */
	let mut delayed_fib = extra::future::Future::spawn(proc() fibonacci::fibonacci(n));
	println("Doing something else");
	println!("fib({:d}) = {}", n, delayed_fib.get());

	/*
	 * The workload will be distributed on the available cores.
	 */
	let mut futures = vec::from_fn(1000, |ind| extra::future::Future::spawn( proc() { partial_sum(ind) }));

	let mut final_res = 0f64;
	for ft in futures.mut_iter() {
		final_res += ft.get();
	}
	println!("π^2/6 is not far from : {}", final_res);
}
