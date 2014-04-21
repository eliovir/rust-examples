#![crate_id="design_pattern-templatemethod#1.0"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-0.11-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-21

use std::fmt;

struct Duck {
	name: ~str,
	weight: f64,
}
impl Duck {
	fn new(name: ~str, weight: f64) -> Duck {
		Duck { name: name, weight: weight }
	}
}
impl fmt::Show for Duck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "The duck {} weights {:.2f} kg.", self.name, self.weight)
	}
}
// To order a Vec, we need to implement TotalOrd to call sort()
// http://static.rust-lang.org/doc/master/std/vec/struct.Vec.html
// http://static.rust-lang.org/doc/master/std/cmp/trait.TotalOrd.html
impl Eq for Duck {
    #[inline]
    fn eq(&self, other: &Duck) -> bool {
        match self.cmp(other) { Equal => true, _ => false }
    }
}
impl TotalEq for Duck {}
impl Ord for Duck {
    #[inline]
    fn lt(&self, other: &Duck) -> bool {
        match self.cmp(other) { Less => true, _ => false}
    }
}
impl TotalOrd for Duck {
	#[inline]
	fn cmp(&self, other: &Duck) -> Ordering {
		if self.weight < other.weight {
			return Less; 
		}
		if self.weight > other.weight {
			return Greater;
		}
		if self.name < other.name {
			return Less;
		}
		if self.name > other.name {
			return Greater;
		}
		return Equal;
        }
}
fn main() {
	let mut ducks = vec!(
		Duck::new(~"Daffy", 8f64),
		Duck::new(~"Dewey", 2f64),
		Duck::new(~"Howard", 7f64),
		Duck::new(~"Louie", 2f64),
		Duck::new(~"Donald", 10f64),
		Duck::new(~"Huey", 2f64)
	);

	println!("Before sorting:");
	display(&ducks);

	ducks.sort();

	println!("After sorting:");
	display(&ducks);
}
fn display<'a>(ducks: &'a Vec<Duck>) {
	for duck in ducks.iter() {
		println!("{}", duck);
	}
}
