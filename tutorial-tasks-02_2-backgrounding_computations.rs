/**
 * Rust Tasks and Communication Tutorial - 2.2 Backgrounding computations: Futures
 * http://static.rust-lang.org/doc/0.7/tutorial-tasks.html#backgrounding-computations-futures
 */
extern mod extra;

extern mod fibonacci(name = "fibonacci", vers = "1.0", author = "eliovir");

fn main() {
	let n = 40;
	println("Setting spawn");
	/*
	 * Note that the future needs to be mutable so that it can save the result for next time get is called.
	 */
	let mut delayed_fib = extra::future::spawn(|| fibonacci::fibonacci(n));
	println("Doing something else");
	println(fmt!("fib(%d) = %?", n, delayed_fib.get()))
}
