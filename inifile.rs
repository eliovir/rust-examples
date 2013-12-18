#[link(name = "configparser", vers = "1.0", author = "eliovir")];
#[crate_type = "lib"];
#[license = "MIT"];
#[desc = "Library for simple INI file management" ];
#[comment = "Example of library: INI file management"];
/**
 * INI file management, API from Python
 *
 * Tested with rust-0.9-pre
 *
 * @author Eliovir <http://github.com/~eliovir>
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 * @see http://docs.python.org/2/library/configparser.html
 * @since 2013-12-18
 */

extern mod extra;

use std::hashmap::HashMap;
use std::io::buffered::BufferedReader;
use std::io::fs::File;
use std::path::Path;

pub struct IniFile {
	path: Path,
	opts: HashMap<~str, HashMap<~str, ~str>>,
	sections: ~[~str]
}

impl IniFile {
	//add_section(section)
	pub fn filepath(&self) -> ~str {
		format!("{}", self.path.display())
	}
	pub fn get(&self, section: ~str, option: ~str) -> ~str {
		if !self.has_section(section.clone()) {
			()
		}
		if !self.opts.get(&section).contains_key(&option) {
			()
		}
		self.opts.get(&section).get(&option).to_owned()
	}
	//has_option(section, option)
	pub fn has_section(&self, section: ~str) -> bool {
		self.opts.contains_key(&section)
	}
	//items(section)
	pub fn new() -> IniFile {
		IniFile { path: Path::new(""), opts: HashMap::new(), sections: ~[]}
	}
	pub fn parseString(&mut self, lines: ~[~str]) {
		let mut section: ~str = ~"Default";
		for line in lines.iter() {
			let mut line_len = line.len();
			if line_len > 0 && line.slice_chars(line_len - 1, line_len) == "\n" {
				line_len = line_len - 1;
			}
			if line_len == 0 {
				continue;
			}
			if line.slice_chars(0, 1) == "#" ||
			   line.slice_chars(0, 1) == ";" {
				continue;
			}
			if line.slice_chars(0, 1) == "[" {
				section = line.slice_chars(1, line_len - 1).to_owned();
				if !self.opts.contains_key(&section) {
					self.opts.insert(section.clone(), HashMap::new());
					self.sections.push(section.clone());
				}
				continue;
			}
			let index: uint = line.find_str("=").unwrap();
			let optkey: ~str = line.slice_chars(0, index).to_owned();
			let optval: ~str = line.slice_chars(index + 1, line_len).to_owned();
			self.opts.get_mut(&section).insert(optkey, optval);
		}
	}
	pub fn read(&mut self, filepath: ~str) {
		self.path = Path::new(filepath);
		let on_error = || fail!("open of {:?} failed", self.path);
		let file : File = File::open(&self.path).unwrap_or_else(on_error);
		let mut reader = BufferedReader::new(file);
		let mut lines: ~[~str] = ~[];
		for line in reader.lines() {
		    lines.push(line);
		}
		self.parseString(lines);
	}
	//remove_option(section, option)
	//remove_section(section)
	pub fn sections(&self) -> ~[~str] {
		/*
		let mut sections: ~[~str] = ~[];
		self.opts.iter().advance(|(k, _)| {sections.push(k.to_owned()); true});
		sections
		*/
		self.sections.clone()
	}
	//set(section, option, value)
	//write(fileobject)
}

#[cfg(test)]
mod tests {
	#[test]
	fn defaultFilepathIsEmpty() {
		let ini = ::IniFile::new();
		let expected=".";
		let found=ini.filepath();
		assert!(expected==found, format!("Default file path must be \"\", not \"{}\".", found));
	}
	#[test]
	fn filepath() {
		let mut ini = ::IniFile::new();
		let filepath=~"data/config.ini";
		ini.read(filepath);
		let expected="data/config.ini";
		let found=ini.filepath();
		assert!(expected==found, format!("Default file path must be \"{}\", not \"{}\".", expected, found));
	}
	#[test]
	fn sections_length() {
		let mut ini = ::IniFile::new();
		ini.read(~"data/config.ini");
		let expected = 3;
		let sections = ini.sections();
		let found = sections.len();
		assert!(expected==found, format!("{:u} sections are expected, not {:u}.", expected, found));
	}
	#[test]
	fn sections_names() {
		let mut ini = ::IniFile::new();
		ini.read(~"data/config.ini");
		let expected = ~[~"section1", ~"section2", ~"Booleans"];
		let found = ini.sections();
		assert!(expected==found, format!("Sections must be \"{:?}\", not {:?}.", expected, found));
	}
	#[test]
	fn has_section_true() {
		let mut ini = ::IniFile::new();
		ini.read(~"data/config.ini");
		let found = ini.has_section(~"section1");
		assert!(found, "Section section1 must be found!");
	}
	#[test]
	fn has_section_false() {
		let mut ini = ::IniFile::new();
		ini.read(~"data/config.ini");
		let found = ini.has_section(~"unknown section");
		assert!(!found, "Section \"unknown section\" must not be found!");
	}
	#[test]
	fn get() {
		let mut ini = ::IniFile::new();
		ini.read(~"data/config.ini");
		let expected = "string 11";
		let found = ini.get(~"section1", ~"value11");
		assert!(expected==found, format!("[section1] value11 must be \"{}\", not \"{}\".", expected, found));
	}
	#[test]
	fn get_bool() {
		assert!(false);
	}
	#[test]
	fn get_int() {
		assert!(false);
	}
	#[test]
	fn get_f64() {
		assert!(false);
	}
}
#[bench]
fn bench_inifile(b: &mut extra::test::BenchHarness) {
	b.iter(|| {
		let mut ini = IniFile::new();
		ini.read(~"data/config.ini");
	});
}
