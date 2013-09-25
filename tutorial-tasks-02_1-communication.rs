/**
 * Rust Tasks and Communication Tutorial - 2.1 Communication
 *
 * Originally, snippets from the Dave Herman's presentation (2013-01-06) about concurrency
 * http://www.infoq.com/presentations/Rust
 *
 * Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
 *
 * Corrected to 0.7
 * http://static.rust-lang.org/doc/0.7/tutorial-tasks.html#communication
 */
struct Point {
    x: float,
    y: float
}
fn main() {
	let (port, channel): (Port<~Point>, Chan<~Point>) = stream();
	// isolate process using spawn
	do spawn || {
		let s = ~Point { x: 1.0, y: 2.0 };
		// the channel moves the pointer
		channel.send(s);
	}
	let s = port.recv();
	assert!(s.x == 1.0);
	assert!(s.y == 2.0);
}