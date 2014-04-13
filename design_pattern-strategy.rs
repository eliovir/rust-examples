#![crate_id="design_pattern-strategy#1.0"]
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
//! @since 2014-04-12

// Variations are encapsulated into external objects.
// Here, it is the fly function.
trait FlyBehaviour {
	fn fly(&self);
}

struct FlyWithWings;

impl FlyBehaviour for FlyWithWings {
	fn fly(&self) {
		println!("I can fly using my wings!");
	}
}

struct DoNotFly;

impl FlyBehaviour for DoNotFly {
	fn fly(&self) {
		println!("I can't fly!");
	}
}

// The object has reference to the variation.
struct Duck {
	flyBehaviour: ~FlyBehaviour,
}

impl Duck {
	// a method calls the funciton in the variation.
	fn fly(&self) {
		self.flyBehaviour.fly();
	}
	fn setFlyBehaviour(&mut self, flyBehaviour: ~FlyBehaviour) {
		self.flyBehaviour = flyBehaviour;
	}
}

fn main() {
	let dnf = DoNotFly;
	let fww = FlyWithWings;
	let mut ducky = Duck { flyBehaviour: ~fww };
	ducky.fly();
	// so functions can change dynamically
	ducky.setFlyBehaviour(~dnf);
	ducky.fly();
}
