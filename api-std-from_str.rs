/**
 * Converting from String
 * http://doc.rust-lang.org/std/from_str/trait.FromStr.html
 * http://doc.rust-lang.org/std/primitive.str.html#method.parse
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 **/
use std::num::ParseIntError;

fn main() {
	/*
	 * When converting from string to numbers, you will have to provide the type manually
	 */
	let f: f32 = "1.2".parse().ok().expect("Wrong format!");
	assert_eq!(f, 1.2f32);
	// or
	let i : i32 = "5".parse().unwrap();
	assert_eq!(i, 5);

	let oi: Result<u32, ParseIntError> = "1".parse();
	assert_eq!(oi, Ok(1));
	// Error, if the provided string cannot be converted
	let oi: Result<u32, ParseIntError> = "x".parse();
	assert!(oi.is_err());

	let i: u32 = match "1".parse() {
		Ok(value) => value,
		Err(_)    => panic!("oops, expected a number")
	};
	assert_eq!(i, 1);

	let i: u32 = "4".parse().unwrap_or(0);
	assert_eq!(i, 4);
}
