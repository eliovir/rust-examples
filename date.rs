#![crate_name = "date"]
#![crate_type = "lib"]
//! Date management
//!
//! Use "constructor", string manipulation
//! Tested with rust-1.3.0
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2013-10-24
//!
//! @todo : get_day_of_week(), get_week(), comparisons
use std::fmt;

/**
 * Simple struct to handle date.
 */
pub struct Date {
	day: u32,
	month: u32,
	year: u32
}

impl Date {
	/**
	 * Add days to the current day. Use negative to remove day.
	 */
	pub fn add_days(&mut self, days: i32) {
		let mut day = self.day as i32;
		let mut month = self.month;
		let mut year = self.year;
		day = day + days;
		if days > 0 {
			let mut month_length = Date::month_length(year, month) as i32;
			while day > month_length {
				day = day - month_length;
				month = month + 1;
				if month > 12 {
					year = year + 1;
					month = 1;
				}
				month_length = Date::month_length(year, month) as i32;
			}
		}
		if day == 0 {
			month = month - 1;
			if month < 1 {
				month = 12;
				year = year - 1;
			}
			day = Date::month_length(year, month) as i32;
		}
		if days < 0 {
			while day < 1 {
				month = month - 1;
				if month < 1 {
					year = year - 1;
					month = 12;
				}
				let month_length = Date::month_length(year, month) as i32;
				day = day + month_length;
			}
		}
		self.day = day as u32;
		self.month = month;
		self.year = year;
	}

	/**
	 * Get day of year.
	 */
	pub fn get_day_of_year(&self) -> u32 {
		let mut doy = self.day;
		for month in 1.. self.month {
			doy += Date::month_length(self.year, month);
		}
		doy
	}

	/**
	 * Check if defined date is valid.
	 */
	pub fn is_valid(&self) -> bool {
		if self.month < 1 || self.month > 12 {
			false
		} else if self.day < 1 || self.day > Date::month_length(self.year, self.month) {
			false
		} else {
			true
		}
	}

	/*
	 * Static methods
	 */

	/**
	 * Static method to know if the year is a leap year.
	 */
	pub fn is_leap(year: u32) -> bool {
		(year % 4 == 0 && year % 100 != 0) || year % 400 == 0
	}

	/**
	 * Static method to get the number of days in the month.
	 */
	pub fn month_length(year: u32, month: u32) -> u32 {
		match  month {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			2 => if Date::is_leap(year) { 29 } else { 28 },
			4 | 6 | 9 | 11 => 30,
			_ => panic!("Wrong month")
		}
	}

	/**
	 * "Constructor".
	 */
	pub fn new(year: u32, month: u32, day: u32) -> Date {
		Date{day: day, month: month, year: year}
	}

	/**
	 * "Constructor" using string like "2013-10-24".
	 */
	pub fn new_from_string(string: &str) -> Date {
		if string.len() < 10 {
			panic!("Wrong format!");
		}
		let year : u32 = string[0..4].parse().ok().expect("Wrong format!");
		let month : u32 = string[5..7].parse().ok().expect("Wrong format!");
		let day : u32 = string[8..10].parse().ok().expect("Wrong format!");
		Date{day: day, month: month, year: year}
	}
}

/**
 * Operator overloading
 *
 * @see http://maniagnosis.crsr.net/2013/04/operator-overloading-in-rust.html
 */
impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:4}-{:2}-{:2}", self.year, self.month, self.day)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn add_days() {
		let mut date = ::Date::new(2013, 10, 24);
		let orig = date.to_string();
		let days = 2;
		date.add_days(days);
		let expected = "2013-10-26".to_string();
		let found = date.to_string();
		assert!(expected==found, "Adding {} days to {} should return {}, not {}", days, orig, expected, found);
	}
	#[test]
	fn get_day_of_year() {
		let date = ::Date::new(2014, 01, 01);
		let expected = 1;
		let found = date.get_day_of_year();
		assert!(expected==found, "{} must be day number {} of the year, not {}.", date, expected, found);
		let date = ::Date::new(2012, 12, 31);
		let expected = 366;
		let found = date.get_day_of_year();
		assert!(expected==found, "{} must be day number {} of the year, not {}.", date, expected, found);
	}
	#[test]
	fn is_leap() {
		assert!(!::Date::is_leap(1900), "1900 is not a leap year");
		assert!(!::Date::is_leap(1901), "1901 is not a leap year");
		assert!(::Date::is_leap(2000), "2000 is leap year");
		assert!(::Date::is_leap(2004), "2004 is leap year");
	}
	#[test]
	fn is_valid() {
		let mut date = ::Date::new(2013, 10, 24);
		assert!(date.is_valid(), "2013-10-24 is a valid date");
		date = ::Date::new(2013, 02, 29);
		assert!(!date.is_valid(), "2013-02-29 isn't a valid date");
		date = ::Date::new(2012, 02, 29);
		assert!(date.is_valid(), "2012-02-29 isn't a valid date");
	}
	#[test]
	fn to_string() {
		let date = ::Date::new(2013, 10, 24);
		let expected = "2013-10-24";
		let found = date.to_string();
		assert!(expected == found, "{}!={}", expected, found);
	}
	/*
	 * Static methods
	 */
	#[test]
	fn month_length() {
		assert!(::Date::month_length(2000, 2) == 29, "February 2000 has 29 days");
		assert!(::Date::month_length(2001, 2) == 28, "February 2001 has 28 days");
		assert!(::Date::month_length(2013, 2) == 28, "February 2013 has 28 days");
		assert!(::Date::month_length(2013, 9) == 30, "September 2013 has 30 days");
		assert!(::Date::month_length(2013, 10) == 31, "October 2013 has 31 days");
	}
	#[test]
	fn new() {
		let date = ::Date::new(2013, 10, 24);
		let expected = "2013-10-24".to_string();
		let found = date.to_string();
		assert!(expected == found, "{}!={}", expected, found);
	}
	#[test]
	fn new_from_string() {
		let date = ::Date::new_from_string("2013-10-24 23:24:34");
		let expected = "2013-10-24".to_string();
		let found = date.to_string();
		assert!(expected == found, "{}!={}", expected, found);
	}
}

