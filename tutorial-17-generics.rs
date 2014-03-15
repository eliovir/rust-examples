/**
 * 17 Generics
 * http://static.rust-lang.org/doc/master/tutorial.html#generics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::vec_ng::Vec;

fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> Vec<U> {
	let mut accumulator: Vec<U> = Vec::new();
	for element in vector.iter() {
		accumulator.push(function(element));
	}
	return accumulator;
}
fn main() {
	let strings = ~["a", "b", "c"];
	let new_strings = map(strings, |&x| { x + x });
	println!("{:?} -> {:?}", strings, new_strings);
}
