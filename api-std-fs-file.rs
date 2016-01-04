/**
 * http://doc.rust-lang.org/std/fs/struct.File.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {

	let mut f = File::open("foo.txt").ok().expect("foo.txt not open");
	let mut buffer = Vec::new();
	let result = f.read_to_end(&mut buffer);
	println!("size: {}", result.ok().expect("foo.txt not read"));

	/*
	 * Read a complete file
	 */
	let path = Path::new("inifile/src/data/config.ini");
	let mut hw_file = File::open(&path).ok().expect("file not open");
	let mut data = Vec::new();
	match hw_file.read_to_end(&mut data) {
		Ok(s) => println!("size: {}, data: {:?}", s, data),
		Err(e) => panic!("error while reading {} : {}", path.to_str().unwrap(), e)
	}

	/*
	 * Iterate over the lines of a file
	 */
	let hw_file = File::open(&path).ok().expect("file not open");
	let file = BufReader::new(hw_file);
	for line in file.lines() {
		match line {
			Ok(nread) => println!("{}", nread),
			Err(e) => println!("error reading: {}", e)
		}
	}
}
