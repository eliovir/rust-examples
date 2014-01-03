#[crate_id="inifile#1.0"];
#[crate_type = "lib"];
#[license = "MIT"];
#[desc = "Library for simple INI file management" ];
#[comment = "Example of library: INI file management"];
/**
 * INI file management, partial implementation of Python API.
 *
 * Tested with rust-0.9-pre
 *
 * @author Eliovir <http://github.com/~eliovir>
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 * @see http://docs.python.org/2/library/configparser.html
 * @since 2013-12-18
 * @todo eddyb: you may want that to be Option<&'a str> so you can return None when the option isn't present. Option<T> can be either Some(T) or None. Option<~T> and Option<&T> are nullable pointers semantically (and optimized as such)
 */

extern mod extra;

use std::from_str::FromStr;
use std::hashmap::HashMap;
use std::io::buffered::BufferedReader;
use std::io::fs::File;
use std::path::Path;

pub struct IniFile {
	/// Comments on sections and options
	comments: HashMap<~str, HashMap<~str, ~str>>,
	/// Option names, used to keep order (as HashMap doesn't).
	options: ~[~[~str]],
	/// INI structure: sections contain options (name=>value)
	opts: HashMap<~str, HashMap<~str, ~str>>,
	/// File path
	path: Path,
	/// Section names, used to keep order (as HashMap doesn't).
	sections: ~[~str]
}

/**
 * IniFile implements a basic configuration which provides a structure similar to what's found in Microsoft Windows INI files.
 * You can use this to write programs which can be customized by end users easily.
 */
impl IniFile {
	/**
	 * Add a section named section to the instance.
	 * If a section by the given name already exists, fail!()
	 */
	pub fn add_section(&mut self, section: &str) {
		if !self.has_section(section.to_owned()) {
			self.comments.insert(section.to_owned(), HashMap::new());
			self.opts.insert(section.to_owned(), HashMap::new());
			self.sections.push(section.to_owned());
			self.options.push(~[]);
		} else {
			fail!("The section {:?} already exists!", section);
		}
	}
	/**
	 * Getter on filepath.
	 */
	pub fn filepath(&self) -> ~str {
		format!("{}", self.path.display())
	}
	/**
	 * Get an option value for the named section.
	 */
	pub fn get(&self, section: &str, option: &str) -> ~str {
	//pub fn get<'a>(&'a self, section: &str, option: &str) -> &'a str {
		if !self.has_option(section, option) {
			()
		}
		self.opts.get(&section.to_owned()).get(&option.to_owned()).to_owned()
	}
	/**
	 * A convenience method which coerces the option in the specified section to a boolean.
	 * Note that the accepted values for the option are '1', 'yes', 'true', and 'on', which cause this method to return True, and '0', 'no', 'false', and 'off', which cause it to return False.
	 * @todo These string values are checked in a case-insensitive manner. 
	 */
	pub fn get_bool(&self, section: &str, option: &str) -> bool {
		let value = self.get(section, option);
		match value {
			~"1" | ~"yes" | ~"true" | ~"T" | ~"on" => true,
			~"0" | ~"no" | ~"false" | ~"F" | ~"off" => false,
			_ => fail!("{} is not a boolean.", value)
		}
	}
	/**
	 * A convenience method which coerces the option in the specified section to a float f64.
	 */
	pub fn get_f64(&self, section: &str, option: &str) -> f64 {
		let value = self.get(section, option);
		let x: Option<f64> = FromStr::from_str(value);
		match x {
			None => fail!("{} is not a float.", value),
			_ => x.unwrap()
		}
	}
	/**
	 * A convenience method which coerces the option in the specified section to an integer.
	 */
	pub fn get_int(&self, section: &str, option: &str) -> int {
		let value = self.get(section, option);
		// https://github.com/mozilla/rust/wiki/Doc-FAQ-Cheatsheet#string-to-int
		let x: Option<int> = FromStr::from_str(value);
		match x {
			None => fail!("{} is not an integer.", value),
			_ => x.unwrap()
		}
	}
	/**
	 * Indicates whether the given section exists and contains the given option.
	 */
	pub fn has_option(&self, section: &str, option: &str) -> bool {
		self.has_section(section) &&
			self.opts.get(&section.to_owned()).contains_key(&option.to_owned())
	}
	/**
	 * Indicates whether the named section is present in the configuration.
	 */
	pub fn has_section(&self, section: &str) -> bool {
		self.opts.contains_key(&section.to_owned())
	}
	pub fn new() -> IniFile {
		IniFile { comments: HashMap::new(), options: ~[~[]], path: Path::new(""), opts: HashMap::new(), sections: ~[]}
	}
	/**
	 * Return a list of options available in the specified section.
	 */
	pub fn options(&self, section: ~str) -> ~[~str] {
		let section_index = self.sections.position_elem(&section).unwrap();
		self.options[section_index].clone()
	}
	/**
	 * Read and parse configuration data from filepath.
	 */
	pub fn read(&mut self, filepath: &str) {
		self.path = Path::new(filepath);
		let on_error = || fail!("open of {:?} failed", self.path);
		let file: File = File::open(&self.path).unwrap_or_else(on_error);
		let mut reader = BufferedReader::new(file);
		let mut lines: ~[~str] = ~[];
		for line in reader.lines() {
			lines.push(line);
		}
		self.read_string(lines);
	}
	/**
	 * Parse configuration data from a vector of strings (file lines).
	 */
	pub fn read_string(&mut self, lines: ~[~str]) {
		let mut section: ~str = ~"Default";
		let mut comment_lines: ~str = ~"";
		for line in lines.iter() {
			let mut line_len = line.len();
			if line_len > 0 && line.slice_chars(line_len - 1, line_len) == "\n" {
				line_len = line_len - 1;
			}
			if line_len == 0 {
				comment_lines.push_str(line.clone());
				continue;
			}
			if line.slice_chars(0, 1) == "#" ||
			line.slice_chars(0, 1) == ";" {
				comment_lines.push_str(line.clone());
				continue;
			}
			if line.slice_chars(0, 1) == "[" {
				section = line.slice_chars(1, line_len - 1).to_owned();
				if !self.opts.contains_key(&section) {
					self.add_section(section.clone());
					self.comments.get_mut(&section).insert(~"__section_comment__", comment_lines);
					comment_lines = ~"";
				}
				continue;
			}
			let index: uint = line.find_str("=").unwrap();
			let optkey: ~str = line.slice_chars(0, index).to_owned();
			let optval: ~str = line.slice_chars(index + 1, line_len).to_owned();
			self.comments.get_mut(&section).insert(optkey.clone(), comment_lines);
			comment_lines = ~"";
			self.opts.get_mut(&section).insert(optkey.clone(), optval);
			let section_index = self.sections.position_elem(&section).unwrap();
			self.options[section_index].push(optkey.clone());
		}
	}
	/**
	 * Remove the specified option from the specified section. If the section does not exist, fails.
	 * If the option existed to be removed, return True; otherwise return False.
	 */
	 pub fn remove_option(&mut self, section: ~str, option: ~str) -> bool {
		if !self.has_section(section.clone()) {
			fail!("Section [{:?}] does not exist!");
		}
	/*
		if !self.has_option(section.to_owned(), option.to_owned()) {
			false
		}
	*/
		let section_index = self.sections.position_elem(&section).unwrap();
		self.options[section_index].remove(section_index);
		self.comments.get_mut(&section).pop(&option);
		self.opts.get_mut(&section).pop(&option);
	 	true
	 }
	/**
	 * Remove the specified section from the configuration.
	 * If the section in fact existed, return True; otherwise return False.
	 */
	pub fn remove_section(&mut self, section: ~str) -> bool {
	/*
		if (!self.has_section(section.clone())) {
			false
		}
	*/
		self.opts.pop(&section);
		self.comments.pop(&section);
		// http://static.rust-lang.org/doc/0.8/std/vec.html
		let index = self.sections.position_elem(&section).unwrap();
		self.sections.remove(index);
		self.options.remove(index);
		true
	}
	/**
	 * Save the current configuration into the original file.
	 */
	pub fn save(&self) {
		self.write(self.filepath());
	}
	/**
	 * Return a list of the available sections.
	 */
	pub fn sections(&self) -> ~[~str] {
		/*
		let mut sections: ~[~str] = ~[];
		self.opts.iter().advance(|(k, _)| {sections.push(k.to_owned()); true});
		sections
		*/
		self.sections.clone()
	}
	/**
	 * If the given section exists, set the given option to the specified value; otherwise fail!().
	 */
	pub fn set(&mut self, section: ~str, option: ~str, value: ~str) {
		if !self.has_section(section.to_owned()) {
			//self.add_section(section.to_owned());
			fail!("Section [{:?}] does not exist!");
		}
		if !self.has_option(section.to_owned(), option.to_owned()) {
			self.opts.get_mut(&section).insert(option.clone(), value);
			let section_index = self.sections.position_elem(&section).unwrap();
			self.options[section_index].push(option.clone());
		} else {
			self.opts.get_mut(&section).swap(option, value);
		}
	}
	/**
	 * Redefine file path.
	 */
	pub fn set_path(&mut self, filepath: Path) {
		self.path = filepath;
	}
	/**
	 * Write a representation of the configuration to the specified file path.
	 * This representation can be parsed by a future read() call.
	 */
	pub fn write(&self, filepath: &str) {
		// http://static.rust-lang.org/doc/master/std/io/index.html
		let mut file = File::create(&Path::new(filepath));
		file.write(self.to_str().into_bytes());
	}
}

/**
 * Operator overloading
 * @see http://maniagnosis.crsr.net/2013/04/operator-overloading-in-rust.html
 */
impl ToStr for IniFile {
	/**
	 * Converts the value of self to an owned string.
	 */
	fn to_str(&self) -> ~str {
		let mut lines = ~"";
		let sections = self.sections().clone();
		for section in sections.iter() {
			if self.comments.contains_key(section) && self.comments.get(section).contains_key(&~"__section_comment__") {
				lines.push_str(self.comments.get(section).get(&~"__section_comment__").clone());
			}
			lines.push_str(format!("[{}]\n", section.to_owned()));
			let options = self.options(section.to_owned()).clone();
			for key in options.iter() {
				if self.comments.contains_key(section) && self.comments.get(section).contains_key(key) {
					lines.push_str(self.comments.get(section).get(key).clone());
				}
				lines.push_str(format!("{}={}\n", key.to_owned(), self.get(section.to_owned(), key.to_owned())));
			}
		}
		lines
	}
}

#[cfg(test)]
mod tests {
	use std::hashmap::HashMap;
	use std::io::buffered::BufferedReader;
	use std::io::fs;
	use std::io::fs::File;
	use std::path::Path;
	#[test]
	fn defaultFilepathIsEmpty() {
		let ini = super::IniFile::new();
		let expected = ".";
		let found = ini.filepath();
		assert!(expected == found, format!("Default file path must be \"\", not \"{}\".", found));
	}
	#[test]
	fn filepath() {
		let mut ini = super::IniFile::new();
		let filepath = "data/config.ini";
		ini.read(filepath);
		let expected = "data/config.ini";
		let found=ini.filepath();
		assert!(expected == found, format!("Default file path must be \"{}\", not \"{}\".", expected, found));
	}
	#[test]
	fn sections_length() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let expected = 5;
		let sections = ini.sections();
		let found = sections.len();
		assert!(expected == found, format!("{:u} sections are expected, not {:u}.", expected, found));
	}
	#[test]
	fn sections_names() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let expected = ~[~"section1", ~"section2", ~"Booleans", ~"Integers", ~"Floats"];
		let found = ini.sections();
		assert!(expected == found, format!("Sections must be \"{:?}\", not {:?}.", expected, found));
	}
	#[test]
	fn has_option_true() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let found = ini.has_option("section1", "value11");
		assert!(found, "Option \"value11\" in section [section1] must be found!");
	}
	#[test]
	fn has_option_false() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let found = ini.has_option("section1", "unknown key");
		assert!(!found, "Option \"unknown key\" in section [section1] must not be found!");
	}
	#[test]
	fn has_section_true() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let found = ini.has_section("section1");
		assert!(found, "Section section1 must be found!");
	}
	#[test]
	fn has_section_false() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let found = ini.has_section("unknown section");
		assert!(!found, "Section \"unknown section\" must not be found!");
	}
	#[test]
	fn get() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let expected = "string 11";
		let found = ini.get("section1", "value11");
		assert!(expected == found, format!("[section1] value11 must be \"{}\", not \"{}\".", expected, found));
	}
	#[test]
	fn get_bool_true() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let options = ["true1", "true2", "true3"];
		for key in options.iter() {
			let found = ini.get_bool("Booleans", key.to_owned());
			assert!(found, format!("[Booleans] {:?} must be true.", key));
		}
	}
	#[test]
	fn get_bool_false() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let options = ["false1", "false2", "false3"];
		for key in options.iter() {
			let found = ini.get_bool("Booleans", key.to_owned());
			assert!(!found, format!("[Booleans] {:?} must be false.", key));
		}
	}
	#[test]
	#[should_fail]
	fn get_bool_fail() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		ini.get_bool("section1", "value11");
	}
	#[test]
	fn get_int() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let mut test: HashMap<~str, int> = HashMap::new();
		test.insert(~"integer0", 0i);
		test.insert(~"integer1", 1i);
		test.insert(~"integer2", 2i);
		test.insert(~"integer3", 03i);
		for (key, expected) in test.iter() {
			let found = ini.get_int("Integers", key.to_owned());
			assert!((expected*1) == found,
				format!("[Integers] {:?} must be \"{:?}\", not \"{:?}\".", key, expected, found));
		}
	}
	#[test]
	#[should_fail]
	fn get_int_fail() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		ini.get_int("section1", "value11");
	}
	#[test]
	fn get_f64() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let mut test: HashMap<~str, f64> = HashMap::new();
		test.insert(~"float01", 0.1f64);
		test.insert(~"float11", 1.1f64);
		test.insert(~"float20", 2.0f64);
		test.insert(~"float30", 3.0f64);
		for (key, expected) in test.iter() {
			let found = ini.get_f64("Floats", key.to_owned());
			assert!((expected*1.0f64) == found,
				format!("[Floats] {:?} must be \"{:?}\", not \"{:?}\".", key, expected, found));
		}
	}
	#[test]
	fn add_section() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let expected = ~[~"section1", ~"section2", ~"Booleans", ~"Integers", ~"Floats"];
		let found = ini.sections();
		assert!(expected == found, format!("Sections must be \"{:?}\", not {:?}.", expected, found));
		ini.add_section("New section");
		let expected2 = ~[~"section1", ~"section2", ~"Booleans", ~"Integers", ~"Floats", ~"New section"];
		let found2 = ini.sections();
		assert!(expected2 == found2, format!("Sections must be \"{:?}\", not {:?}.", expected2, found2));
	}
	#[test]
	#[should_fail]
	fn add_section_twice() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		ini.add_section("New section");
		ini.add_section("New section");
	}
	#[test]
	fn remove_section() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		ini.remove_section(~"section1");
		let expected = ~[~"section2", ~"Booleans", ~"Integers", ~"Floats"];
		let found = ini.sections();
		assert!(expected == found, format!("Sections must be \"{:?}\", not {:?}.", expected, found));
	}
	#[test]
	fn set() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		ini.set(~"section1", ~"value2", ~"string 2");
		let expected = "string 2";
		let found = ini.get("section1", "value2");
		assert!(expected == found, format!("[section1] value2 must be \"{}\", not \"{}\".", expected, found));
	}
	#[test]
	fn options() {
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let expected = ~[~"value11", ~"value"];
		let found = ini.options(~"section1");
		assert!(expected == found, format!("Items of [section1] must be \"{:?}\", not {:?}.", expected, found));
	}
	#[test]
	fn to_str() {
		let filepath = "data/config.ini";
		let path = Path::new(filepath);
		let on_error = || fail!("open of {:?} failed", path);
		let file: File = File::open(&path).unwrap_or_else(on_error);
		let mut reader = BufferedReader::new(file);
		let mut lines: ~[~str] = ~[];
		for line in reader.lines() {
			lines.push(line);
		}
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		let found = ini.to_str();
		let expected = lines.concat();
		assert_eq!(expected, found);
	}
	#[test]
	fn write() {
		use std::task;
		use std::any::Any;
		// Copy config.ini to write_test.ini using `write()`.
		let writepath = "data/write_test.ini";
		let mut ini = super::IniFile::new();
		ini.read("data/config.ini");
		ini.write(writepath);

		// Check that the new file exists
		let path = Path::new(writepath);
		assert!(path.exists(), format!("{} should exist after writing inifile!", writepath));

		// Check the contents
		let mut ini2 = super::IniFile::new();
		ini2.read(writepath);
		let found = ini2.to_str();
		let expected = ini.to_str();
		assert_eq!(expected, found);
		
		// Clean
		let result: Result<(), ~Any> = do task::try {
			fs::unlink(&Path::new(writepath));
		};
		assert!(!result.is_err(), format!("Unlinking {} should not fail!", writepath));
	}
	#[test]
	fn save() {
		let filepath = ~"data/write_test.ini";
		let path = Path::new(filepath);

		let mut ini = super::IniFile::new();
		ini.add_section("section1");
		ini.set(~"section1", ~"key1", ~"value1");
		ini.set_path(path.clone());
		ini.save();


		let on_error = || fail!("open of {:?} failed", path);
		let file: File = File::open(&path).unwrap_or_else(on_error);
		let mut reader = BufferedReader::new(file);
		let mut lines: ~[~str] = ~[];
		for line in reader.lines() {
			lines.push(line);
		}

		let found = lines.concat();
		let expected = ~"[section1]\nkey1=value1\n";
		assert_eq!(expected, found);
		fs::unlink(&path);
	}
}
#[bench]
fn bench_inifile(b: &mut extra::test::BenchHarness) {
	b.iter(|| {
		let mut ini = IniFile::new();
		ini.read("data/config.ini");
	});
}
