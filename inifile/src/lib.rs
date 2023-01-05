#![crate_name = "inifile"]
#![crate_type = "lib"]
//! INI file management, partial implementation of Python API.
//!
//! Tested with rust-0.12-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @see http://docs.python.org/2/library/configparser.html
//!
//! @since 2013-12-18
//!
//! @todo eddyb: you may want that to be Option<&'a str> so you can return None when the option isn't present. Option<T> can be either Some(T) or None. Option<~T> and Option<&T> are nullable pointers semantically (and optimized as such)

#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct IniFile<'a> {
    /// Comments on sections and options
    comments: HashMap<String, HashMap<String, String>>,
    /// Option names, used to keep order (as HashMap doesn't).
    options: Vec<Vec<String>>,
    /// INI structure: sections contain options (name=>value)
    opts: HashMap<String, HashMap<String, String>>,
    /// File path
    path: &'a Path,
    /// Section names, used to keep order (as HashMap doesn't).
    sections: Vec<String>,
}

/**
 * IniFile implements a basic configuration which provides a structure similar to what's found in Microsoft Windows INI files.
 * You can use this to write programs which can be customized by end users easily.
 */
impl<'a> IniFile<'a> {
    /**
     * Add a section named section to the instance.
     * If a section by the given name already exists, panic!()
     */
    pub fn add_section(&mut self, section: &str) {
        if !self.has_section(section) {
            self.comments.insert(section.to_string(), HashMap::new());
            self.opts.insert(section.to_string(), HashMap::new());
            self.sections.push(section.to_string());
            self.options.push(Vec::new());
        } else {
            panic!("The section {:?} already exists!", section);
        }
    }
    /**
     * Getter on filepath.
     */
    pub fn filepath(&self) -> String {
        format!("{}", self.path.display())
    }
    /**
     * Get an option value for the named section.
     */
    //pub fn get<'a>(&self, section: &'a str, option: &'a str) -> String {
    pub fn get(&self, section: &str, option: &str) -> String {
        if !self.has_option(section, option) {
            ()
        }
        self.opts[section][option].clone()
    }
    /**
     * A convenience method which coerces the option in the specified section to a boolean.
     * Note that the accepted values for the option are '1', 'yes', 'true', and 'on', which cause this method to return True, and '0', 'no', 'false', and 'off', which cause it to return False.
     * @todo These string values are checked in a case-insensitive manner.
     */
    pub fn get_bool(&self, section: &str, option: &str) -> bool {
        let value = self.get(section, option);
        match &*value {
            "1" | "yes" | "true" | "T" | "on" => true,
            "0" | "no" | "false" | "F" | "off" => false,
            _ => panic!("{} is not a boolean.", value),
        }
    }
    /**
     * A convenience method which coerces the option in the specified section to a float f64.
     */
    pub fn get_f64(&self, section: &str, option: &str) -> f64 {
        let value = self.get(section, option);
        match value.trim().parse() {
            Ok(f) => f,
            Err(_) => panic!("{} is not a float.", value),
        }
    }
    /**
     * A convenience method which coerces the option in the specified section to an integer.
     */
    pub fn get_int(&self, section: &str, option: &str) -> isize {
        let value = self.get(section, option);
        match value.trim().parse() {
            Ok(f) => f,
            Err(_) => panic!("{} is not an integer.", value),
        }
    }
    /**
     * Indicates whether the given section exists and contains the given option.
     */
    pub fn has_option(&self, section: &str, option: &str) -> bool {
        self.has_section(section) && self.opts[section].contains_key(option)
    }
    /**
     * Indicates whether the named section is present in the configuration.
     */
    pub fn has_section(&self, section: &str) -> bool {
        self.opts.contains_key(&section.to_string())
    }
    pub fn new() -> IniFile<'a> {
        IniFile {
            comments: HashMap::new(),
            options: Vec::new(),
            path: Path::new(""),
            opts: HashMap::new(),
            sections: Vec::new(),
        }
    }
    /**
     * Return a list of options available in the specified section.
     */
    pub fn options(&self, section: String) -> Vec<String> {
        match self.sections.iter().position(|x| x == &section) {
            //match self.sections.iter().position_elem(&section) {
            Some(section_index) => self.options[section_index].to_vec(),
            None => {
                vec![]
            }
        }
    }
    /**
     * Read and parse configuration data from filepath.
     */
    pub fn read(&mut self, filepath: &'a str) {
        self.path = Path::new(filepath);
        match File::open(&self.path) {
            Err(e) => panic!("open of {:?} failed: {}", self.path, e),
            Ok(file) => {
                debug!("open of {:?} succeeded", self.path);
                let reader = BufReader::new(file);
                let mut lines: Vec<String> = Vec::new();
                for line in reader.lines() {
                    match line {
                        Ok(nread) => lines.push(nread),
                        Err(e) => println!("error reading: {}", e),
                    }
                }
                self.read_string(lines);
            }
        }
    }
    /**
     * Parse configuration data from a vector of strings (file lines).
     */
    pub fn read_string(&mut self, lines: Vec<String>) {
        let mut section: String = "Default".to_string();
        let mut comment_lines = String::new();
        for line in lines.iter() {
            let mut line_len = line.len();
            let line_slice = line;
            if line_len > 0 && &line_slice[line_len - 1..line_len] == "\n" {
                line_len = line_len - 1;
            }
            if line_len == 0 {
                comment_lines.push_str(line_slice);
                comment_lines.push_str("\n");
                continue;
            }
            if &line_slice[0..1] == "#" || &line_slice[0..1] == ";" {
                comment_lines.push_str(line_slice);
                comment_lines.push_str("\n");
                continue;
            }
            if &line_slice[0..1] == "[" {
                section = line_slice[1..line_len - 1].to_string();
                if !self.opts.contains_key(&section) {
                    self.add_section(&section);
                    self.comments
                        .get_mut(&section)
                        .unwrap()
                        .insert("__section_comment__".to_string(), comment_lines);
                    comment_lines = String::new();
                }
                continue;
            }
            let index = line_slice.find("=").unwrap();
            let optkey = line_slice[0..index].to_string();
            let optval = line_slice[index + 1..line_len].to_string();
            self.comments
                .get_mut(&section)
                .unwrap()
                .insert(optkey.clone(), comment_lines);
            comment_lines = String::new();
            self.opts
                .get_mut(&section)
                .unwrap()
                .insert(optkey.clone(), optval);
            let section_index = self.sections.iter().position(|x| x == &section).unwrap();
            self.options
                .get_mut(section_index)
                .unwrap()
                .push(optkey.clone());
        }
    }
    /**
     * Remove the specified option from the specified section. If the section does not exist, fails.
     * If the option existed to be removed, return True; otherwise return False.
     */
    pub fn remove_option(&mut self, section: String, option: String) -> bool {
        if !self.has_section(&section) {
            panic!("Section [{:?}] does not exist!", section);
        }
        /*
            if !self.has_option(section.to_string(), option.to_string()) {
                false
            }
        */
        let section_index = self.sections.iter().position(|x| x == &section).unwrap();
        self.options
            .get_mut(section_index)
            .unwrap()
            .remove(section_index);
        self.comments.get_mut(&section).unwrap().remove(&option);
        self.opts.get_mut(&section).unwrap().remove(&option);
        true
    }
    /**
     * Remove the specified section from the configuration.
     * If the section in fact existed, return True; otherwise return False.
     */
    pub fn remove_section(&mut self, section: String) -> bool {
        /*
            if (!self.has_section(section.clone())) {
                false
            }
        */
        self.opts.remove(&section);
        self.comments.remove(&section);
        match self.sections.iter().position(|x| x == &section) {
            Some(index) => {
                self.sections.remove(index);
                self.options.remove(index);
                true
            }
            None => false,
        }
    }
    /**
     * Save the current configuration into the original file.
     */
    pub fn save(&self) {
        self.write(&self.filepath());
    }
    /**
     * Return a list of the available sections.
     */
    pub fn sections(&self) -> Vec<String> {
        /*
        let mut sections: ~[~str] = ~[];
        self.opts.iter().advance(|(k, _)| {sections.push(k.to_string()); true});
        sections
        */
        self.sections.clone()
    }
    /**
     * If the given section exists, set the given option to the specified value; otherwise panic!().
     */
    pub fn set(&mut self, section: String, option: String, value: String) {
        if !self.has_section(&section) {
            panic!("Section [{:?}] does not exist!", section);
        }
        if !self.has_option(&section, &option) {
            self.opts
                .get_mut(&section)
                .unwrap()
                .insert(option.clone(), value);
            let section_index = self.sections.iter().position(|x| x == &section).unwrap();
            self.options
                .get_mut(section_index)
                .unwrap()
                .push(option.clone());
        } else {
            self.opts.get_mut(&section).unwrap().insert(option, value);
        }
    }
    /**
     * Redefine file path.
     */
    pub fn set_path(&mut self, filepath: &'a Path) {
        self.path = filepath;
    }
    /**
     * Write a representation of the configuration to the specified file path.
     * This representation can be parsed by a future read() call.
     */
    pub fn write(&self, filepath: &str) {
        // http://doc.rust-lang.org/std/fs/struct.File.html
        match File::create(&Path::new(filepath)) {
            Ok(mut file) => match file.write_all(self.to_string().as_bytes()) {
                Ok(()) => debug!("INI file {:?} written", self.path),
                Err(e) => println!("failed to write to {:?}: {}", self.path, e),
            },
            Err(e) => println!("failed to create {:?}: {}", self.path, e),
        }
    }
}

/**
 * Formatting trait <https://doc.rust-lang.org/std/fmt/>.
 * Operator overloading
 * @see http://maniagnosis.crsr.net/2013/04/operator-overloading-in-rust.html
 */
impl<'a> fmt::Display for IniFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = String::new();
        let sections = self.sections().clone();
        for section in sections.iter() {
            if self.comments.contains_key(section)
                && self.comments[section].contains_key(&"__section_comment__".to_string())
            {
                lines.push_str(&self.comments[section]["__section_comment__"]);
            }
            let line = format!("[{}]\n", section);
            lines.push_str(&line);
            let options = self.options(section.clone()).clone();
            for key in options.iter() {
                if self.comments.contains_key(section) && self.comments[section].contains_key(key) {
                    lines.push_str(&self.comments[section][key]);
                }
                lines.push_str(&format!("{}={}\n", key.to_string(), self.get(section, key)));
            }
        }
        write!(f, "{}", lines)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    static FILEPATH: &'static str = "src/data/config.ini";

    /**
     * Path::exists() is unstable in Rust-1.3.0.
     */
    fn exists(path: &Path) -> bool {
        match fs::metadata(path) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    #[test]
    fn default_filepath_is_empty() {
        let ini = super::IniFile::new();
        let expected = "".to_string();
        let found = ini.filepath();
        assert!(
            expected == found,
            format!(
                "Default file path must be \"{}\", not \"{}\".",
                expected, found
            )
        );
    }
    #[test]
    fn filepath() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let expected = FILEPATH.to_string();
        let found = ini.filepath();
        assert!(
            expected == found,
            format!(
                "Default file path must be \"{}\", not \"{}\".",
                expected, found
            )
        );
    }
    #[test]
    fn sections_length() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let expected = 5;
        let sections = ini.sections();
        let found = sections.len();
        assert!(
            expected == found,
            format!("{} sections are expected, not {}.", expected, found)
        );
    }
    #[test]
    fn sections_names() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let expected = vec![
            "section1".to_string(),
            "section2".to_string(),
            "Booleans".to_string(),
            "Integers".to_string(),
            "Floats".to_string(),
        ];
        let found = ini.sections();
        assert!(
            expected == found,
            format!("Sections must be \"{:?}\", not {:?}.", expected, found)
        );
    }
    #[test]
    fn has_option_true() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let found = ini.has_option("section1", "value11");
        assert!(
            found,
            "Option \"value11\" in section [section1] must be found!"
        );
    }
    #[test]
    fn has_option_false() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let found = ini.has_option("section1", "unknown key");
        assert!(
            !found,
            "Option \"unknown key\" in section [section1] must not be found!"
        );
    }
    #[test]
    fn has_section_true() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let found = ini.has_section("section1");
        assert!(found, "Section section1 must be found!");
    }
    #[test]
    fn has_section_false() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let found = ini.has_section("unknown section");
        assert!(!found, "Section \"unknown section\" must not be found!");
    }
    #[test]
    fn get() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let expected = "string 11".to_string();
        let found = ini.get("section1", "value11");
        assert!(
            expected == found,
            format!(
                "[section1] value11 must be \"{}\", not \"{}\".",
                expected, found
            )
        );
    }
    #[test]
    fn get_bool_true() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let options = ["true1", "true2", "true3"];
        for key in options.iter() {
            let found = ini.get_bool("Booleans", key);
            assert!(found, format!("[Booleans] {:?} must be true.", key));
        }
    }
    #[test]
    fn get_bool_false() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let options = ["false1", "false2", "false3"];
        for key in options.iter() {
            let found = ini.get_bool("Booleans", key);
            assert!(!found, format!("[Booleans] {:?} must be false.", key));
        }
    }
    #[test]
    #[should_panic]
    fn get_bool_fail() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        ini.get_bool("section1", "value11");
    }
    #[test]
    fn get_int() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let mut test: HashMap<String, isize> = HashMap::new();
        test.insert("integer0".to_string(), 0isize);
        test.insert("integer1".to_string(), 1isize);
        test.insert("integer2".to_string(), 2isize);
        test.insert("integer3".to_string(), 03isize);
        for (key, expected) in test.iter() {
            let found = ini.get_int("Integers", key);
            assert!(
                (expected * 1) == found,
                format!(
                    "[Integers] {:?} must be \"{:?}\", not \"{:?}\".",
                    key, expected, found
                )
            );
        }
    }
    #[test]
    #[should_panic]
    fn get_int_fail() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        ini.get_int("section1", "value11");
    }
    #[test]
    fn get_f64() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let mut test: HashMap<String, f64> = HashMap::new();
        test.insert("float01".to_string(), 0.1f64);
        test.insert("float11".to_string(), 1.1f64);
        test.insert("float20".to_string(), 2.0f64);
        test.insert("float30".to_string(), 3.0f64);
        for (key, expected) in test.iter() {
            let found = ini.get_f64("Floats", key);
            assert!(
                (expected * 1.0f64) == found,
                format!(
                    "[Floats] {:?} must be \"{:?}\", not \"{:?}\".",
                    key, expected, found
                )
            );
        }
    }
    #[test]
    fn add_section() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let expected = vec![
            "section1".to_string(),
            "section2".to_string(),
            "Booleans".to_string(),
            "Integers".to_string(),
            "Floats".to_string(),
        ];
        let found = ini.sections();
        assert!(
            expected == found,
            format!("Sections must be \"{:?}\", not {:?}.", expected, found)
        );
        ini.add_section("New section");
        let expected2 = vec![
            "section1".to_string(),
            "section2".to_string(),
            "Booleans".to_string(),
            "Integers".to_string(),
            "Floats".to_string(),
            "New section".to_string(),
        ];
        let found2 = ini.sections();
        assert!(
            expected2 == found2,
            format!("Sections must be \"{:?}\", not {:?}.", expected2, found2)
        );
    }
    #[test]
    #[should_panic]
    fn add_section_twice() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        ini.add_section("New section");
        ini.add_section("New section");
    }
    #[test]
    fn remove_section() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        ini.remove_section("section1".to_string());
        let expected = vec![
            "section2".to_string(),
            "Booleans".to_string(),
            "Integers".to_string(),
            "Floats".to_string(),
        ];
        let found = ini.sections();
        assert!(
            expected == found,
            format!("Sections must be \"{:?}\", not {:?}.", expected, found)
        );
    }
    #[test]
    fn set() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        ini.set(
            "section1".to_string(),
            "value2".to_string(),
            "string 2".to_string(),
        );
        let expected = "string 2".to_string();
        let found = ini.get("section1", "value2");
        assert!(
            expected == found,
            format!(
                "[section1] value2 must be \"{}\", not \"{}\".",
                expected, found
            )
        );
    }
    #[test]
    fn options() {
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let expected = vec!["value11".to_string(), "value".to_string()];
        let found = ini.options("section1".to_string());
        assert!(
            expected == found,
            format!(
                "Items of [section1] must be \"{:?}\", not {:?}.",
                expected, found
            )
        );
    }
    #[test]
    fn to_string() {
        let path = Path::new(FILEPATH);
        let file = File::open(&path);
        match file {
            Err(e) => panic!("open of {:?} failed: {}", path, e),
            _ => debug!("open of {:?} succeeded", path),
        }
        let reader = BufReader::new(file.unwrap());
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(nread) => lines.push(nread + "\n"),
                Err(e) => println!("error reading: {}", e),
            }
        }
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        let found = ini.to_string();
        let expected = lines.concat();
        assert_eq!(expected, found);
    }
    #[test]
    fn write() {
        // Copy config.ini to write_test.ini using `write()`.
        let writepath = "src/data/write_test.ini";
        let mut ini = super::IniFile::new();
        ini.read(FILEPATH);
        ini.write(writepath);

        // Check that the new file exists
        let path = Path::new(writepath);
        assert!(
            exists(path),
            format!("{} should exist after writing inifile!", writepath)
        );

        // Check the contents
        let mut ini2 = super::IniFile::new();
        ini2.read(writepath);
        let found = ini2.to_string();
        let expected = ini.to_string();
        assert_eq!(expected, found);

        // Clean
        assert!(
            exists(path),
            format!("{} should exist after reading the new inifile!", writepath)
        );
        let result = fs::remove_file(&path);
        assert!(
            !result.is_err(),
            format!("Unlinking {} should not panic!", writepath)
        );
    }
    #[test]
    fn save() {
        let filepath = "src/data/save_test.ini";
        let path = Path::new(filepath);
        if exists(path) {
            match fs::remove_file(&path) {
                Err(e) => panic!("removing {:?} failed: {}", path, e),
                _ => debug!("removing {:?} succeeded", path),
            }
        }

        let mut ini = super::IniFile::new();
        ini.add_section("section1");
        ini.set(
            "section1".to_string(),
            "key1".to_string(),
            "value1".to_string(),
        );
        ini.set_path(path.clone());
        ini.save();

        let file = File::open(&path);
        match file {
            Err(e) => panic!("open of {:?} failed: {}", path, e),
            _ => debug!("open of {:?} succeeded", path),
        }
        let reader = BufReader::new(file.unwrap());
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(nread) => lines.push(nread + "\n"),
                Err(e) => println!("error reading: {}", e),
            }
        }

        let found = lines.concat();
        let expected = "[section1]\nkey1=value1\n".to_string();
        assert_eq!(expected, found);
        match fs::remove_file(&path) {
            Err(e) => panic!("removing {:?} failed: {}", path, e),
            _ => debug!("removing {:?} succeeded", path),
        }
    }
}
