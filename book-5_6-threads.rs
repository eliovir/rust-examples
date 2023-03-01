//! Concurrency
//! <http://doc.rust-lang.org/book/concurrency.html>
//!
//! was
//! Rust Tasks and Communication Tutorial - 2 Basics
//! <http://doc.rust-lang.org/guide-tasks.html#basics>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn generate_task_number() -> isize {
	10
}
fn main() {
	// Print something profound in a different task using a named function
	fn print_message() { println!("I am running in a different task!"); }
	thread::spawn(print_message);

	// Print something more profound in a different task using a lambda expression
	thread::spawn(|| { println!("I am also running in a different task!") });

	// It returns a handle to the thread, that can be used to wait for the child thread to finish and extract its result:
	let handle = thread::spawn(|| {
		"Hello from a thread!"
	});
	let result = handle.join();
	assert!(!result.is_err());
	println!("{}", result.unwrap());

	// Generate some state locally
	let child_task_number_10 = generate_task_number();

	// to force the closure to take ownership of `child_task_number` (and any other referenced
	// variables), use the `move` keyword
	thread::spawn(move || {
		   // Capture it in the remote task
		   println!("I am child number {}", child_task_number_10);
	});

	for child_task_number in 0u32..20 {
		thread::spawn(move || {
			print!("I am child number {}\n", child_task_number);
		});
	}

	//
	let data = Arc::new(Mutex::new(0));
	let (tx, rx) = mpsc::channel();

	for _ in 0..10 {
		let (data, tx) = (data.clone(), tx.clone());
		thread::spawn(move || {
			// lock(), which acquires the mutex's lock.
			let mut data = data.lock().unwrap();
			*data += 1;
			let _unused = tx.send(*data);
		});
	}
	for _ in 0..10 {
		println!("{}", rx.recv().unwrap());
	}
}
