/**
 * Converting from String 
 * http://static.rust-lang.org/doc/master/std/from_str/trait.FromStr.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 **/

fn main() {
	/*
	 * When converting from string to numbers, you will have to provide the type manually
	 */
	let f: f32 = from_str("1.2").unwrap();
	assert_eq!(f, 1.2f32);
	// or
	let i = from_str::<uint>("5").unwrap();
	assert_eq!(i, 5);

	let oi: Option<uint> = from_str("1");
	assert_eq!(oi, Some(1u));
	// None, if the provided string cannot be converted
	let oi: Option<uint> = from_str("x");
	assert_eq!(oi, None);

	let i: uint = match from_str("1") {
		Some(value) => value,
		None        => fail!("oops, expected a number")
	};
	assert_eq!(i, 1u);

	let i: uint = from_str("4").unwrap_or(0u);
	assert_eq!(i, 4u);
}
