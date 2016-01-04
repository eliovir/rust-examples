//! Script to handle CSV terminology, reading CSV terminology, printing TBX entries.
//!
//! Tested with rust-1.3.0
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>

use std::fmt::Debug;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::path::Path;

// The generic `T` must implement `Debug`. So regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("<!-- DEBUG :: {:?} -->", t);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let path = {
		if args.len() == 2 {
			Path::new(&args[1])
		} else {
			Path::new("data/datumbazo.csv")
		}
	};
	let reader = match File::open(&path) {
		Err(e) => panic!("open of {:?} failed: {}", path, e),
		Ok(file) => {
			print_debug(&format!("open of {:?} succeeded", path));
			BufReader::new(file)
		}
	};
	let mut count = 0u32;
	let langs = vec!("-", "en_US", "eo", "nl", "fr", "de", "es", "ca", "pl");
	for line in reader.lines() {
		if count == 0 {
			count += 1;
			continue;
		}
		match line {
			Ok(nread) => {
				count += 1;
				if nread.starts_with("\t") {
					continue;
				}
				let v: Vec<&str> = nread.split("\t").map(|s| s.trim()).collect();
				print!("
      <termEntry id=\"komputeko-{}\">
        <descrip type=\"subjectField\">computer</descrip>", count);
				let mut l = 0usize;
				for word in v.iter() {
					if l == 0 {
						l += 1;
						continue;
					}
					if word.len() == 0 {
						l += 1;
						continue;
					}
					print!("
        <langSet xml:lang=\"{}\">
          <ntig>
            <termGrp>
              <term>{}</term>
            </termGrp>
          </ntig>
        </langSet>", langs.get(l).unwrap(), word);
					l += 1;
				}
				print!("
      </termEntry>");
			},
			Err(e) => println!("error reading: {}", e)
		}
	}
}
