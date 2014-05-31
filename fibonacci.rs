#![crate_id="fibonacci#1.1"]
#![crate_type = "lib"]
#![license = "MIT"]
#![desc = "Fibonacci library" ]
#![comment = "Example of library: compute Fibonacci"]
#![deny(missing_doc)]
//! Compute fibonacci.
//!
//! The Fibonacci sequence is a sequence of numbers starting with 0 and 1 where every subsequent number is the sum of the previous two numbers.
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @author Eliovir <http://github.com/~eliovir>

extern crate test;

use std::mem::replace;

#[cfg(test)]
use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
#[cfg(test)]
static BENCH_SIZE: uint = 20;
#[cfg(test)]
static BENCH_SIZE_INT: int = 20;

/**
* Calcule les elements de la suite de Fibonnaci.
*
* REGLE RG024 Le projet permet de calculer les membres de la Suite de
* Fibonacci.
*
* REGLE RG024.1 : f(1) = 1
* REGLE RG024.2 : f(2) = 1
*
* REGLE RG024.3 : f(n) = f(n-1) + f(n-2) si n > 2
*
* REGLE RG024.4 : il n'est pas possible de calculer la valeur de la Suite
* de Fibonacci pour un rang negatif.
*
* REGLE RG024.5 : le calcul de n'importe quel element de la Suite de
* Fibonacci doit s'effectuer en moins de deux secondes.
*
* REGLE RG024.6 : le calcul de n'importe quel element de la Suite de
* Fibonacci, pour un rang inferieur a 50, doit s'effectuer en moins d'une
* seconde.
*
* @param n le rang pour lequel on calcule le membre.
* @return Le membre de rang n dans la Suite.
*/
pub fn fibonacci_reccursive(n: int) -> uint {
	if n < 0 {
		fail!("{:d} is negative!", n);
	}
	match n {
		0     => fail!("zero is not a right argument to fibonacci_reccursive()!"),
		1 | 2 => 1,
		3     => 2,
		/*
		50    => 12586269025,
		*/
		_     => fibonacci(n - 1) + fibonacci(n - 2)
	}
}

/**
 * Non reccursive function.
 */
pub fn fibonacci(n: int) -> uint {
	if n < 0 {
		fail!("{:d} is negative!", n);
	} else if n == 0 {
		fail!("zero is not a right argument to fibonacci()!");
	} else if n == 1 {
		return 1u;
	}
	let mut i = 0;
	let mut sum = 0;
	let mut last = 0;
	let mut curr = 1;
	while i < n - 1 {
		sum = last + curr;
		last = curr;
		curr = sum;
		i += 1;
	}
	sum
}

/**
 * Iiterative fibonacci
 * https://github.com/japaric/rust-by-example
 */
pub struct Fibonacci {
	curr: uint,
	next: uint,
}

impl Iterator<uint> for Fibonacci {
	fn next(&mut self) -> Option<uint> {
		let new_next = self.curr + self.next;
		let new_curr = replace(&mut self.next, new_next);

		Some(replace(&mut self.curr, new_curr))
	}
}

/**
 * A "constructor" for Iiterative fibonacci
 */
pub fn iterative_fibonacci() -> Fibonacci {
	Fibonacci { curr: 1, next: 1 }
}

#[cfg(test)]
fn RG024_x(n: int, expected: uint) {
	let mut found = fibonacci_reccursive(n);
	assert!(expected == found, format!("fibibonacci_reccursive({:d}): expected ({:u}) != found ({:u})", n, expected, found));
	found = fibonacci(n);
	assert!(expected == found, format!("fibibonacci({:d}): expected ({:u}) != found ({:u})", n, expected, found));
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE RG024.1 : f(1) = 1
 */
#[test]
fn RG024_1() {
	RG024_x(1, 1);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE RG024.2 : f(2) = 1
 */
#[test]
fn RG024_2() {
	RG024_x(2, 1);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE RG024.3.a : f(3) = 2
 */
#[test]
fn RG024_3_a() {
	RG024_x(3, 2);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE RG024.4.a : il n'est pas possible de calculer la valeur de la Suite
 * de Fibonacci pour un rang negatif ou nul.
 */
#[test]
#[should_fail]
fn RG024_4_a() {
	fibonacci(-1);
	fibonacci(0);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE RG024.5 : f(55) = 139583862445
 */
#[test]
fn RG024_5() {
	/*RG024_x(55, 139583862445);*/
	RG024_x(30, 832040);
}
#[bench]
fn bench_fibonacci_reccursive(b: &mut Bencher) {
	b.iter(|| {
		fibonacci_reccursive(BENCH_SIZE_INT);
	});
}
#[bench]
fn bench_fibonacci(b: &mut Bencher) {
	b.iter(|| {
		fibonacci(BENCH_SIZE_INT);
	});
}
#[bench]
fn bench_iterative_fibonacci(b: &mut Bencher) {
	b.iter(|| {
		// http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
		iterative_fibonacci().take(BENCH_SIZE).last().unwrap()
		//iterative_fibonacci().take(BENCH_SIZE).collect::<Vec<uint>>()
	})
}
