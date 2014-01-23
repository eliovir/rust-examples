#[crate_id = "date#1.0"];
#[crate_type = "lib"];
#[license = "MIT"];
#[desc = "Library for simple date management" ];
#[comment = "Example of library: date management"];
/**
 * Date management
 *
 * Use "constructor", string manipulation
 * Tested with rust-0.9-pre
 *
 * @author Eliovir <http://github.com/~eliovir>
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 * @since 2013-10-24
 */
use std::from_str::FromStr;

pub struct Date {
	day: int,
	month: int,
	year: int
}

impl Date {
	pub fn addDays(&mut self, days: int) {
		let mut day = self.day;
		let mut month = self.month;
		let mut year = self.year; 
		day = day + days;
		if days > 0 {
			while day > Date::monthLength(year, month) {
				day = day - Date::monthLength(year, month);
				month = month + 1;
				if (month > 12) {
					year = year + 1;
					month = 1;
				}
			}
		}
		if day == 0 {
			month = month - 1;
			if month < 1 {
				month = 12;
				year = year - 1;
			}
			day = Date::monthLength(year, month);
		}
		if days < 0 {
			while day < 1 {
				month = month - 1;
				if month < 1 {
					year = year - 1;
					month = 12;
				}
				day = day + Date::monthLength(year, month);
			}
		}
		self.day = day;
		self.month = month;
		self.year = year;
	}

	pub fn isValid(&self) -> bool {
		if self.month < 1 || self.month > 12 {
			false
		} else if self.day < 1 || self.day > Date::monthLength(self.year, self.month) {
			false
		} else {
			true
		}
	}

	/*
	 * Static methods
	 */
	pub fn isLeap(year: int) -> bool {
		(year % 4 == 0 && year % 100 != 0) || year % 400 == 0
	}
	pub fn monthLength(year: int, month: int) -> int {
		match  month {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			2 => if Date::isLeap(year) { 29 } else { 28 },
			4 | 6 | 9 | 11 => 30,
			_ => fail!("Wrong month")
		}
	}
	pub fn new(year: int, month: int, day: int) -> Date {
		Date{day: day, month: month, year: year}
	}
	pub fn newFromString(string: &str) -> Date {
		if string.len() < 10 {
			fail!("Wrong format!");
		}
		let year = FromStr::from_str(string.slice_chars(0, 4)).unwrap();
		let month = FromStr::from_str(string.slice_chars(5, 7)).unwrap();
		let day = FromStr::from_str(string.slice_chars(8, 10)).unwrap();
		Date{day: day, month: month, year: year}
	}
}

/**
 * Operator overloading
 * @see http://maniagnosis.crsr.net/2013/04/operator-overloading-in-rust.html
 */
impl ToStr for Date {
	fn to_str(&self) -> ~str {
		format!("{:4d}-{:2d}-{:2d}", self.year, self.month, self.day)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn addDays() {
		let mut date = ::Date::new(2013, 10, 24);
		let orig = date.to_str();
		let days = 2;
		date.addDays(days);
		let expected="2013-10-26";
		let found=date.to_str();
		assert!(expected==found, format!("Adding {:d} days to {} should return {}, not {}", days, orig, expected, found));
	}
	#[test]
	fn isLeap() {
		assert!(!::Date::isLeap(1900), "1900 is not a leap year");
		assert!(!::Date::isLeap(1901), "1901 is not a leap year");
		assert!(::Date::isLeap(2000), "2000 is leap year");
		assert!(::Date::isLeap(2004), "2004 is leap year");
	}
	#[test]
	fn isValid() {
		let mut date = ::Date::new(2013, 10, 24);
		assert!(date.isValid(), "2013-10-24 is a valid date");
		date = ::Date::new(2013, 02, 29);
		assert!(!date.isValid(), "2013-02-29 isn't a valid date");
		date = ::Date::new(2012, 02, 29);
		assert!(date.isValid(), "2012-02-29 isn't a valid date");
	}
	#[test]
	fn to_str() {
		let date = ::Date::new(2013, 10, 24);
		let expected = "2013-10-24";
		let found = date.to_str();
		assert!(expected == found, format!("{}!={}", expected, found));
	}
	/*
	 * Static methods
	 */
	#[test]
	fn monthLength() {
		assert!(::Date::monthLength(2000, 2) == 29, "February 2000 has 29 days");
		assert!(::Date::monthLength(2001, 2) == 28, "February 2001 has 28 days");
		assert!(::Date::monthLength(2013, 2) == 28, "February 2013 has 28 days");
		assert!(::Date::monthLength(2013, 9) == 30, "September 2013 has 30 days");
		assert!(::Date::monthLength(2013, 10) == 31, "October 2013 has 31 days");
	}
	#[test]
	fn new() {
		let date = ::Date::new(2013, 10, 24);
		let expected = "2013-10-24";
		let found = date.to_str();
		assert!(expected == found, format!("{}!={}", expected, found));
	}
	#[test]
	fn newFromString() {
		let date = ::Date::newFromString("2013-10-24 23:24:34");
		let expected = "2013-10-24";
		let found = date.to_str();
		assert!(expected == found, format!("{}!={}", expected, found));
	}
}
#[cfg(not(test))]
fn main() {
	println("This program must be build and run with --test");
}


