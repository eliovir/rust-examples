/**
 * 17 Generics
 * http://doc.rust-lang.org/tutorial.html#generics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

fn map<T, U, F>(vector: &[T], function: F) -> Vec<U>
    where F: for<'a> Fn(&'a T) -> U {
	let mut accumulator : Vec<U> = Vec::new();
	for element in vector.iter() {
		accumulator.push(function(element));
	}
	return accumulator;
}
fn main() {
	let strings = ["a", "b", "c"];
	let new_strings = map(&strings, |&x| {let mut msg = String::from(x); msg.push_str(x); msg});
	println!("{:?} -> {:?}", strings, new_strings);
}
