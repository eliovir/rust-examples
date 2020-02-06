//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-1.41.1-nightly
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-12

// This design pattern needs a common trait for Drinks and Ingredients
trait Drinkable {
	fn description(&self) -> String;
	fn price(&self) -> f64;
}

struct Drink<'a> {
	price: f64,
	description: &'a str
}

impl<'a> Drinkable for Drink<'a> {
	fn description(&self) -> String {
		self.description.to_string()
	}
	fn price(&self) -> f64 {
		self.price
	}
}

impl<'a> Drink<'a> {
	// The "constructor", optional but useful!
	pub fn new(description: &'a str, price: f64) -> Drink<'a> {
		Drink { description: description, price: price }
	}
}

struct Ingredient<'a> {
	description: &'a str,
	price: f64,
	// to decorate an struct, it must have the common trait
	drink: Box<dyn Drinkable + 'a>,
}

impl<'a> Drinkable for Ingredient<'a> {
	fn description(&self) -> String {
		let mut description = self.drink.description();
		description.push_str(", ");
		description.push_str(self.description);
		description.clone()
	}
	fn price(&self) -> f64 {
		self.price + self.drink.price()
	}
}

impl<'a> Ingredient<'a> {
	// The "constructor", optional but useful!
	pub fn new(description: &'a str, price: f64, drink: Box<dyn Drinkable + 'a>) -> Ingredient<'a> {
		Ingredient { description: description, price: price, drink: drink }
	}
}

fn main() {
	let columbia = Drink::new("Columbia", 0.89);
	println!("{} => {}", columbia.description(), columbia.price());

	let chocolate = Ingredient::new("chocolate", 0.2, Box::new(columbia));
	println!("{} => {}", chocolate.description(), chocolate.price());

	let chantilly = Ingredient::new("chantilly", 0.1, Box::new(chocolate));
	println!("{} => {}", chantilly.description(), chantilly.price());
}
