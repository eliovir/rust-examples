/**
 * http://static.rust-lang.org/doc/0.8/std/hashmap.html
 * https://github.com/mozilla/rust/blob/0.8/src/test/run-pass/hashmap-memory.rs#L70
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::hashmap::HashMap;

fn main() {
	println("Using borrowed pointers as keys.");
	let mut h: HashMap<&str, int>;
	h = HashMap::new();
	h.insert("foo", 42);
	println(fmt!("Is there a key foo?  => %?", h.contains_key(& &"foo"))); // => true
	println(fmt!("Is there a key baz?  => %?", h.contains_key(& &"baz"))); // => false
	println(fmt!("The value for foo is => %?", h.find(& &"foo"))); // => Some(&42)
	let key = "baz";
	h.insert(key, 1);
	println(fmt!("Is there a key baz?  => %?", h.contains_key(& &"baz"))); // => false

	// You don't actually need the HashMap to own the keys (but
	// unless all keys are static, this will be likely to lead
	// to problems, so I don't suggest you do it in reality)

	println("Using owned pointers as keys.");
	let mut h: HashMap<~str, int> = HashMap::new();
	h.insert(~"foo", 42);
	println(fmt!("Is there a key foo?  => %?", h.contains_key(&~"foo"))); // => true
	println(fmt!("Is there a key baz?  => %?", h.contains_key(&~"baz"))); // => false
	println(fmt!("The value for foo is => %?", h.find(&~"foo"))); // => Some(&42)
	h.insert(key.to_owned(), 1);
	println(fmt!("Is there a key baz?  => %?", h.contains_key(&~"baz"))); // => true
}
