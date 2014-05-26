#![crate_id="find_max"]
//! Test various syntaxes to find the maximum value of a vector using generic function.
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>

#[cfg(test)]
extern crate test;

#[cfg(test)]
use test::Bencher;

/**
 * Code from Rust MeetUp (March, 31st 2014) at Mozilla Space, Paris.
 * https://reps.mozilla.org/e/meetup-rust-paris-02/
 */
fn find_max1<'a, T: Ord>(lst: &'a Vec<T>) -> Option<&'a T> {
	let mut max = None;

	for i in lst.iter() {
		max = match max {
			None => Some(i),
			Some(ref m) if i > *m => Some(i),
			_ => max
		}
	}
	max
}

/**
 * Using a closure.
 *
 * Inspired by tutorial "15 Closure"
 * http://doc.rust-lang.org/tutorial.html#closures
 */
fn find_max2<'a, T: Ord>(lst: &'a Vec<T>) -> Option<&'a T> {
	let mut max = None;

	let find_max = |i: &'a T| {
		max = match max {
			None => Some(i),
			Some(ref m) if i > *m => Some(i),
			_ => max
		}
	};
	for x in lst.iter() {
		find_max(x);
	}

	max
}

/**
 * Using a closure and .map().
 */
fn find_max3<'a, T: Ord>(lst: &'a Vec<T>) -> Option<&'a T> {
	let mut max = None;

	let find_max = |i: &'a T| {
		max = match max {
			None => Some(i),
			Some(ref m) if i > *m => Some(i),
			_ => max
		};
		max
	};
	lst.iter().map(find_max).last().unwrap()
}

/**
 * Using std lib
 */
fn find_maxstd<'a, T: TotalOrd>(lst: &'a Vec<T>) -> Option<&'a T> {
	lst.iter().max_by(|x| *x)
}

#[test]
fn test_find_max1() {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	let nine = 9;
	assert_eq!(Some(&nine), find_max1(&v));
}

#[test]
fn test_find_max2() {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	let nine = 9;
	assert_eq!(Some(&nine), find_max2(&v));
}

#[test]
fn test_find_max3() {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	let nine = 9;
	assert_eq!(Some(&nine), find_max3(&v));
}

#[test]
fn test_find_maxstd() {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	let nine = 9;
	assert_eq!(Some(&nine), find_maxstd(&v));
}

#[bench]
fn bench_find_max1(b: &mut Bencher) {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	b.iter(|| {
		find_max1(&v);
	});
}

#[bench]
fn bench_find_max2(b: &mut Bencher) {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	b.iter(|| {
		find_max2(&v);
	});
}

#[bench]
fn bench_find_max3(b: &mut Bencher) {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	b.iter(|| {
		find_max3(&v);
	});
}

#[bench]
fn bench_find_maxstd(b: &mut Bencher) {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
	b.iter(|| {
		find_maxstd(&v);
	});
}

#[cfg(not(test))]
fn main () {
	let int_v = vec!(5, 2, 0, 8, 2);
	println!("find_max1 -> {}", find_max1(&int_v));
	println!("find_max2 -> {}", find_max2(&int_v));
	println!("find_max3 -> {}", find_max3(&int_v));
	println!("find_maxstd -> {}", find_maxstd(&int_v));
	let v = vec!("qehgesrhsetha", "bqthst", "cthersth");
	let b = find_max3(&v);
	println!("{}", b);

	println!("{}", v);
	println!("{}", b);
}
