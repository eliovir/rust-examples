//! Example of the new operator `?` added in Rust-1.13.0.
//! See <https://blog.rust-lang.org/2016/11/10/Rust-1.13.html>

use std::io::prelude::*;
use std::fs::File;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
	let mut f = File::open("data/username.txt")?;
	let mut s = String::new();

	f.read_to_string(&mut s)?;

	Ok(s)
}

fn read_username_from_file_with_matches() -> Result<String, io::Error> {
	let f = File::open("data/username.txt");
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn read_username_from_file_with_try() -> Result<String, io::Error> {
	let mut f = File::open("data/username.txt")?;
	let mut s = String::new();

	f.read_to_string(&mut s)?;

	Ok(s)
}

#[cfg(test)]
mod tests {
	#[test]
	fn question_mark() {
		let found = ::read_username_from_file();
		assert!(!found.is_err());
		assert_eq!(found.unwrap(), "eliovir\n".to_string());
	}
	#[test]
	fn with_matches() {
		let found = ::read_username_from_file_with_matches();
		assert!(!found.is_err());
		assert_eq!(found.unwrap(), "eliovir\n".to_string());
	}
	#[test]
	fn with_try() {
		let found = ::read_username_from_file_with_try();
		assert!(!found.is_err());
		assert_eq!(found.unwrap(), "eliovir\n".to_string());
	}
}

fn main() {
	match read_username_from_file() {
		Ok(s) => println!("The file `data/username.txt` contains `{}`.", s),
		Err(e) => println!("Something went wrong: {}", e),
	};

	match read_username_from_file_with_matches() {
		Ok(s) => println!("The file `data/username.txt` contains `{}`.", s),
		Err(e) => println!("Something went wrong: {}", e),
	};

	match read_username_from_file_with_try() {
		Ok(s) => println!("The file `data/username.txt` contains `{}`.", s),
		Err(e) => println!("Something went wrong: {}", e),
	};
}
