//! Rust Tasks and Communication Tutorial - 2.3 Sharing immutable data without copy: Arc
//!
//! http://doc.rust-lang.org/guide-tasks.html#sharing-immutable-data-without-copy:-arc
//! Corrected for Rust-1.3.0 with help of Zachary Dremann.

extern crate rand;

use rand::Rng;

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

fn pnorm(nums: &[f64], p: usize) -> f64 {
    nums.iter()
        .fold(0.0, |a, b| a + b.powf(p as f64))
        .powf(1.0 / (p as f64))
}

fn main() {
    let numbers: Vec<f64> = (1..1000000)
        .map(|_| rand::thread_rng().gen_range(1.0, 101.0))
        .collect();
    // With simple pipes, without Arc, a copy would have to be made for each thread.
    //
    // When you clone an Arc<T>, it will create another pointer to the data and increase the
    // reference counter.
    let numbers_arc = Arc::new(numbers);

    let (tx, rx) = channel();
    for num in 1..10 {
        let tx = tx.clone();
        let numbers_arc = numbers_arc.clone();

        thread::spawn(move || {
            let task_numbers = &*numbers_arc;
            tx.send((num, pnorm(task_numbers, num))).unwrap();
        });
    }
    drop(tx);
    for (num, result) in rx.iter() {
        println!("{}-norm = {}", num, result);
    }
}
