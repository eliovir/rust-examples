#![crate_id="design_pattern-observer#1.0"]
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
//! @since 2014-04-18
//! TODO : add/delete/notify observers of different types
//! TODO : display Observable temperature on Observer.update()

// Observer
trait Observer {
	fn update(&self);
}

// Observable memorizes all Observers and send notifications
trait Observable<'a, T: Observer> {
	fn addObserver(&mut self, observer: &'a T);
	fn deleteObserver(&mut self, observer: &'a T);
	fn notifyObservers(&self); 
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
	fn setTemperature(&mut self, temperature: f64) {
		self.temperature = temperature;
		self.notifyObservers();
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
impl std::cmp::Eq for Display {
	fn eq(&self, other: &Display) -> bool {
		self.name == other.name
	}
}
impl std::fmt::Show for Display {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Display {}", self.name)
	}
}
impl<'a, T: Observer+Eq+std::fmt::Show> Observable<'a, T> for Weather<'a, T> {
	fn addObserver(&mut self, observer: &'a T) {
		println!("addObserver({});", observer);
		self.observers.push(observer);
	}
	fn deleteObserver(&mut self, observer: &'a T) {
		let mut index = 0u;
		let mut found = false;
		for &obs in self.observers.iter() {
			if obs == observer {
				println!("deleteObserver({});", observer);
				found = true;
				break;
			}
			index += 1;
		}
		if found {
			self.observers.remove(index);
		}
	}
	fn notifyObservers(&self) {
		for &observer in self.observers.iter() {
			observer.update();
		}
	}
}


fn main() {
	let display = Display::new("Desktop".to_owned());
	let mut weather = Weather{temperature: 19.0, observers: Vec::new()};
	weather.addObserver(&display);
	let display2 = Display::new("Desktop2".to_owned());
	weather.addObserver(&display2);
	weather.setTemperature(20.0);
	weather.deleteObserver(&display2);
	weather.setTemperature(21.0);
}

