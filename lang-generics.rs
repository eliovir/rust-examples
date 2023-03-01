//! Code from "Introduction to Rust" delivered by Felix S. Klock II at the Rust Meetup Paris on Tuesday February 25, 2014.
//! <http://rust-meetup-paris.github.io/Talks/introduction_to_rust/pnkfelix-meetup-2014feb.pdf>
//! <https://github.com/Rust-Meetup-Paris/Talks/tree/master/introduction_to_rust>
//!
//! Tested with rust-1.41.1-nightly
//!
//! @license Creative Commons Attribution 4.0 International License.

/**
 * Functions and data types can be abstracted over types, not just values.
 */
fn safe_get<T>(opt: Option<T>, default: T) -> T {
	match opt {
		Some(contents) => contents,
		None           => default
	}
}

#[derive(Debug)]
struct Dollars {
	amount: u32
}
#[derive(Debug)]
struct Euros {
	amount: u32
}
trait Currency {
	fn render(&self) -> String;
	fn to_euros(&self) -> Euros;
}
/*
 * Traits implementations
 */
impl Currency for Dollars {
	fn render(&self) -> String {
		format!("${}", self.amount)
	}
	fn to_euros(&self) -> Euros {
		let a = (self.amount as f64) * 0.73;
		Euros { amount: a as u32 }
	}
}
impl Currency for Euros {
	fn render(&self) -> String {
		format!("EUR{}", self.amount)
	}
	fn to_euros(&self) -> Euros {
		Euros { amount: self.amount }
	}
}
/*
 * Static resolution
 */
// a and b must have the same types.
fn add_as_euros<C: Currency>(a: &C, b: &C) -> Euros {
	let sum = a.to_euros().amount + b.to_euros().amount;
	Euros{ amount: sum }
}
// To add Dollars and Euros, Generics can not be used.
fn accumeuros(a: &dyn Currency, b: &dyn Currency) -> Euros {
	let sum = a.to_euros().amount + b.to_euros().amount;
	Euros{ amount: sum }
}
fn main() {
	let var1 = Some("contents");
	let default = "default";
	let res1 = safe_get(var1, default);
	println!("safe_get({:?}) -> {:?}", var1, res1);
	let var2 = None;
	let res2 = safe_get(var2, default);
	println!("safe_get({:?}) -> {:?}", var2, res2);

	let eu100 = Euros { amount: 100 };
	let eu200 = Euros { amount: 200 };
	println!("{:?}", add_as_euros(&eu100, &eu200));

	let us100 = Dollars { amount: 100 };
	let us200 = Dollars { amount: 200 };
	println!("{:?}", add_as_euros(&us100, &us200));

	let us100 = Dollars { amount: 100 };
	let eu200 = Euros { amount: 200 };
	println!("{:?}", accumeuros(&us100 as &dyn Currency, &eu200 as &dyn Currency));
}
