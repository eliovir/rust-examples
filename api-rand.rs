extern crate rand;

use rand::Rng;



fn main() {
	let mut tmp_rand = rand::thread_rng();


	let boolean: bool = tmp_rand.gen();
	println!("bool: {}", boolean);


	let int_8: i8 = tmp_rand.gen::<i8>();
	let uint_8: u8 = tmp_rand.gen::<u8>();
	println!("int_8: {}\nuint_8: {}", int_8, uint_8);


	let int_32: i32 = tmp_rand.gen::<i32>();
	let uint_32: u32 = tmp_rand.gen::<u32>();
	println!("int_32: {}\nuint_32: {}", int_32, uint_32);


	let float32: f32 = tmp_rand.gen::<f32>();
	let float64: f64 = tmp_rand.gen::<f64>();
	println!("float32: {}\nfloat64: {}", float32, float64);


	let int8_for_range: u8 = tmp_rand.gen_range(0, 127);
	println!("int8_for_range: {}", int8_for_range);


	let tuple = rand::random::<(f32, f32)>();
	println!("tuple: {:?}", tuple);
}