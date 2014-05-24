#![crate_id="tutorial-tasks-02_3-arc.rs"]
//! Rust Tasks and Communication Tutorial - 2.3 Sharing immutable data without copy: Arc
//!
//! http://doc.rust-lang.org/guide-tasks.html#sharing-immutable-data-without-copy:-arc

extern crate rand;
extern crate sync;

use sync::Arc;

fn pnorm(nums: &[f64], p: uint) -> f64 {
	nums.iter().fold(0.0, |a, b| a + b.powf(p as f64)).powf(1.0 / (p as f64))
}

fn main() {
	let numbers = Vec::from_fn(1000000u, |_| rand::random::<f64>());
	let numbers_arc = Arc::new(numbers);

	for num in range(1u, 10) {
		let (tx, rx) = channel();
		tx.send(numbers_arc.clone());

		spawn(proc() {
			let local_arc : Arc<Vec<f64>> = rx.recv();
			let task_numbers = &*local_arc;
			println!("{}-norm = {}", num, pnorm(task_numbers.as_slice(), num));
		});
	}
}
