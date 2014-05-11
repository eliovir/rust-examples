/**
 * http://static.rust-lang.org/doc/master/collections/hashmap/struct.HashMap.html
 * https://github.com/mozilla/rust/blob/master/src/test/run-pass/hashmap-memory.rs#L70
 * https://github.com/mozilla/rust/blob/master/src/libcollections/hashmap.rs
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern crate collections;
use collections::hashmap::HashMap;

fn main() {
	println!("Using borrowed pointers as keys.");
	let mut h: HashMap<&str, int>;
	h = HashMap::new();
	h.insert("foo", 42);
	println!("Is there a key foo?  => {}", h.contains_key(&("foo"))); // => true
	println!("Is there a key baz?  => {}", h.contains_key(&("baz"))); // => false
	println!("The value for foo is => {:?}", h.find(&("foo"))); // => Some(&42)
	let key = "baz";
	h.insert(key, 1);
	println!("Is there a key baz?  => {}", h.contains_key(&("baz"))); // => false

	// Doing a find, inserting with a `proc()`, using the key to construct the value
	let mut map = HashMap::<~str, ~str>::new();
	map.find_or_insert_with("foo".to_owned(), |k| *k + "bar".to_owned());
	println!("The value for foo is => {:?}", map.find(&("foo".to_owned()))); // => Some(&~"foobar")
	// running this for the first time, will add "foo" with the value 1
	// running the same for the second time, will add +1 to "foo"
	h.insert_or_update_with("foo", 1, |_k, v| *v += 1);
	println!("foo={}", h.get(&("foo")));
	assert_eq!(*h.get(&("foo")), 43);

	// You don't actually need the HashMap to own the keys (but
	// unless all keys are static, this will be likely to lead
	// to problems, so I don't suggest you do it in reality)

	println!("Using owned pointers as keys.");
	let mut h: HashMap<~str, int> = HashMap::new();
	h.insert("foo".to_owned(), 42);
	println!("Is there a key foo?  => {}", h.contains_key(&"foo".to_owned())); // => true
	println!("Is there a key baz?  => {}", h.contains_key(&"baz".to_owned())); // => false
	println!("The value for foo is => {:?}", h.find(&"foo".to_owned())); // => Some(&42)
	h.insert(key.to_owned(), 1);
	println!("Is there a key baz?  => {}", h.contains_key(&"baz".to_owned())); // => true

	// List keys of the HashMap
	let mut keys: Vec<~str> = Vec::new();
	for (k, _) in h.iter() {
		keys.push(k.to_owned());
	}
	println!("These are the keys: {}.", keys);
	let keys = h.keys().map(|v| v.clone()).collect::<Vec<~str>>();
	println!("These are the keys: {}.", keys);

	// List values of the HashMap
	let values = h.values().map(|v| v.clone()).collect::<Vec<int>>();
	println!("These are the values: {}.", values);
}
