//! Rust Tasks and Communication Tutorial - 2.1 Communication
//!
//! Originally, snippets from the Dave Herman's presentation (2013-01-06) about concurrency
//! http://www.infoq.com/presentations/Rust
//!
//! Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
//!
//! Corrected to 1.3.0
//! http://doc.rust-lang.org/guide-tasks.html#communication

use std::sync::mpsc::channel;
use std::thread;

// Simple struct to test data exchange.
struct Point {
    x: f64,
    y: f64
}
fn main() {
	let (tx, rx) = channel();
	// isolate process using spawn
	thread::spawn(move || {
		let s = Point { x: 1.0, y: 2.0 };
		// the channel moves the pointer
		tx.send(s).unwrap();
        println!("Thread end");
	});
	let s = rx.recv().ok().expect("Point not received!");
	assert!(s.x == 1.0);
	assert!(s.y == 2.0);
    println!("Program end");
}
