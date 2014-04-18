//!

// Observer
//trait Observer<U: Observable> {
trait Observer {
	fn update(&mut self);
}

// Observable memorizes all Observers and send notifications
trait Observable<T: Observer> {
	fn addObserver(&mut self, observer: ~T);
	fn deleteObserver(&mut self, observer: ~T);
	fn notifyObservers(&mut self); 
}
//
struct Display {
	name: ~str,
}
struct Weather<T> {
	temperature: f64,
	observers: ~[~T]
}
impl Weather<Display> {
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
	fn update(&mut self) {
		println!("Display {} updated!", self.name);
	}
}
impl Display {
	fn new(name: ~str) -> Display {
		Display{name: name}
	}
}
impl std::cmp::Eq for Display {
	fn eq(&self, other: &Display) -> bool {
		self.name == other.name
	}
}
impl<T: Observer+Eq> Observable<T> for Weather<T> {
	fn addObserver(&mut self, observer: ~T) {
		self.observers.push(observer);
	}
	fn deleteObserver(&mut self, observer: ~T) {
		let mut index = 0u;
		for obs in self.observers.iter() {
			if obs == &observer {
				break;
			}
			index += 1;
		}
		self.observers.remove(index);
	}
	fn notifyObservers(&mut self) {
		for observer in self.observers.mut_iter() {
			observer.update();
		}
	}
}


fn main() {
	let display = Display::new(~"Desktop");
	let mut weather = Weather{temperature: 19.0, observers: ~[]};
	weather.addObserver(~display);
	let display2 = Display::new(~"Desktop2");
	weather.addObserver(~display2);
	weather.setTemperature(20.0);
//	weather.deleteObserver(~display2);
	weather.setTemperature(21.0);
}

