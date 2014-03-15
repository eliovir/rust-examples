/**
 * http://static.rust-lang.org/doc/master/std/io/index.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 **/
use std::io::File;
use std::io::BufferedReader;

fn main() {
	/*
	 * Read a complete file
	 */
	let path = Path::new("data/config.ini");
	let mut hw_file = File::open(&path);
	let data = hw_file.read_to_end();
	println!("{:?}", data);

	/*
	 * Iterate over the lines of a file
	 */
	let mut file = BufferedReader::new(File::open(&path));
	for line in file.lines() {
		match line {
			Ok(nread) => println!("{}", nread),
			Err(e) => println!("error reading: {}", e)
		}
	}
}
