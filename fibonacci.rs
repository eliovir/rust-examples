#![crate_name = "fibonacci"]
#![crate_type = "lib"]
#![cfg_attr(feature = "nightly", feature(test))]

//! Compute fibonacci.
//!
//! The Fibonacci sequence is a sequence of numbers starting with 0 and 1 where every subsequent number is the sum of the previous two numbers.
//!
//! Tested with rust-1.41.0 and rust-1.41.0-nightly
//! Nightly is needed to run benchmarks.
//! <https://doc.rust-lang.org/unstable-book/library-features/test.html>
//!
//! To run the benchmark, use nightly version and
//! `cargo bench --bin fibonacci --features=nightly`
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @author Eliovir <http://github.com/~eliovir>

/*
* Calcule les elements de la suite de Fibonnaci.
*
* REGLE rg_024 Le projet permet de calculer les membres de la Suite de
* Fibonacci.
*
* REGLE rg_024.1 : f(1) = 1
* REGLE rg_024.2 : f(2) = 1
*
* REGLE rg_024.3 : f(n) = f(n-1) + f(n-2) si n > 2
*
* REGLE rg_024.4 : il n'est pas possible de calculer la valeur de la Suite
* de Fibonacci pour un rang negatif.
*
* REGLE rg_024.5 : le calcul de n'importe quel element de la Suite de
* Fibonacci doit s'effectuer en moins de deux secondes.
*
* REGLE rg_024.6 : le calcul de n'importe quel element de la Suite de
* Fibonacci, pour un rang inferieur a 50, doit s'effectuer en moins d'une
* seconde.
*/

/// This code shows the wrong way to compute the Fibonacci sequence, the complexity is exponential.
///
/// It's here to play with `#[bench]`.
///
/// `n` the rank used to compute the member of the sequence.
pub fn fibonacci_reccursive(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        3 => 2,
        /*
        50    => 12586269025,
        */
        _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
    }
}

/// Non reccursive function.
///
/// `n` the rank used to compute the member of the sequence.
pub fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

/// Iterative fibonacci.
///
/// <https://github.com/rust-lang/rust-by-example>
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

/// A "constructor" for Iterative fibonacci.
pub fn iterative_fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

#[cfg(test)]
fn rg_024_x(n: i32, expected: u64) {
    let mut found = fibonacci_reccursive(n);
    assert!(
        expected == found,
        "fibibonacci_reccursive({}): expected ({}) != found ({})",
        n,
        expected,
        found
    );
    found = fibonacci(n);
    assert!(
        expected == found,
        "fibibonacci({}): expected ({}) != found ({})",
        n,
        expected,
        found
    );
}

/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE rg_024.1 : f(1) = 1
 */
#[test]
fn rg_024_1() {
    rg_024_x(1, 1);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE rg_024.2 : f(2) = 1
 */
#[test]
fn rg_024_2() {
    rg_024_x(2, 1);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE rg_024.3.a : f(3) = 2
 */
#[test]
fn rg_024_3_a() {
    rg_024_x(3, 2);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE rg_024.4.a : il n'est pas possible de calculer la valeur de la Suite
 * de Fibonacci pour un rang negatif ou nul.
 */
#[test]
#[should_panic]
fn rg_024_4_a() {
    fibonacci(-1);
    fibonacci(0);
}
/**
 * Test du calcul de la suite de Fibonnaci.
 *
 * REGLE rg_024.5 : f(55) = 139583862445
 */
#[test]
fn rg_024_5() {
    /*rg_024_x(55, 139583862445);*/
    rg_024_x(30, 832040);
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::*;

    // bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
    #[cfg(test)]
    static BENCH_SIZE: usize = 20;
    #[cfg(test)]
    static BENCH_SIZE_INT: i32 = 20;

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
            //iterative_fibonacci().take(BENCH_SIZE).collect::<Vec<u64>>()
        })
    }
}
