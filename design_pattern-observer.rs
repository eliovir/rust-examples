#![crate_name="design_pattern-observer"]
#![crate_version="1.0"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-0.12-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-18
//! TODO : add/delete/notify observers of different types
//! TODO : display Observable temperature on Observer.update()

// Observer
trait Observer {
	fn update(&self);
}

// Observable memorizes all Observers and send notifications
trait Observable<'a, T: Observer> {
	fn add_observer(&mut self, observer: &'a T);
	fn delete_observer(&mut self, observer: &'a T);
	fn notify_observers(&self);
}

// Define Observer and Observable
struct Display {
	name: String,
}
struct Weather<'a, T> {
	temperature: f64,
	observers: Vec<&'a T>
}
impl<'a> Weather<'a, Display> {
	fn set_temperature(&mut self, temperature: f64) {
		self.temperature = temperature;
		self.notify_observers();
	}
}
/*
 * Traits implementations
 */
//impl<U: Observable> Observer<U> for Display<U> {
impl Observer for Display {
	fn update(&self) {
		println!("Display {} updated!", self.name);
	}
}
impl Display {
	fn new(name: String) -> Display {
		Display{name: name}
	}
}
impl std::cmp::PartialEq for Display {
	fn eq(&self, other: &Display) -> bool {
		self.name == other.name
	}
}
impl std::fmt::Show for Display {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Display {}", self.name)
	}
}
impl<'a, T: Observer+PartialEq+std::fmt::Show> Observable<'a, T> for Weather<'a, T> {
	fn add_observer(&mut self, observer: &'a T) {
		println!("add_observer({});", observer);
		self.observers.push(observer);
	}
	fn delete_observer(&mut self, observer: &'a T) {
		let mut index = 0u;
		let mut found = false;
		for &obs in self.observers.iter() {
			if obs == observer {
				println!("delete_observer({});", observer);
				found = true;
				break;
			}
			index += 1;
		}
		if found {
			self.observers.remove(index);
		}
	}
	fn notify_observers(&self) {
		for &observer in self.observers.iter() {
			observer.update();
		}
	}
}

fn main() {
	let display = Display::new("Desktop".to_string());
	let mut weather = Weather{temperature: 19.0, observers: Vec::new()};
	weather.add_observer(&display);
	let display2 = Display::new("Desktop2".to_string());
	weather.add_observer(&display2);
	weather.set_temperature(20.0);
	weather.delete_observer(&display2);
	weather.set_temperature(21.0);
}

