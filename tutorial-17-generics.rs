/**
 * 17 Generics
 * http://doc.rust-lang.org/tutorial.html#generics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> Vec<U> {
	let mut accumulator = Vec::new();
	for element in vector.iter() {
		accumulator.push(function(element));
	}
	return accumulator;
}
fn main() {
	let strings = ["a", "b", "c"];
	let new_strings = map(strings, |&x| { let mut msg = x.to_string(); msg.push_str(x); msg });
	println!("{} -> {}", strings.as_slice(), new_strings);
}
