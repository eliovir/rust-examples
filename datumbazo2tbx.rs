#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Script to handle CSV terminology"]
#![comment = "Reading CSV terminology, printing TBX entries."]
#![feature(phase)]
//! Script to handle CSV terminology, reading CSV terminology, printing TBX entries.
//!
//! Tested with rust-0.11-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
extern crate debug;
#[phase(plugin, link)] extern crate log;

use std::io::BufferedReader;
use std::io::fs::File;
use std::os;

fn main() {
	let args: Vec<String> = os::args();
	let path = {
		if args.len() == 2 {
			Path::new(args.get(1).as_slice())
		} else {
			Path::new("data/datumbazo.csv")
		}
	};
	let mut reader = match File::open(&path) {
		Err(e) => fail!("open of {:?} failed: {}", path, e),
		file => {
			debug!("open of {:?} succeeded", path);
			BufferedReader::new(file)
		}
	};
	let mut count = 0u;
	let langs = vec!("-", "en_US", "eo", "nl", "fr", "de", "es", "ca", "pl");
	for line in reader.lines() {
		if count == 0 {
			count += 1;
			continue;
		}
		match line {
			Ok(nread) => {
				count += 1;
				if nread.as_slice().starts_with("\t") {
					continue;
				}
				let v: Vec<&str> = nread.as_slice().split_str("\t").map(|s| s.trim()).collect();
				print!("
      <termEntry id=\"komputeko-{:u}\">
        <descrip type=\"subjectField\">computer</descrip>", count);
				let mut l = 0u;
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
        </langSet>", langs.get(l), word);
					l += 1;
				}
				print!("
      </termEntry>");
			},
			Err(e) => println!("error reading: {}", e)
		}
	}
}
