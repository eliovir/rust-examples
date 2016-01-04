//!
//! http://doc.rust-lang.org/complement-cheatsheet.html#how-do-i-express-phantom-types?
use std::marker::PhantomData;

// Phantom types are useful for enforcing state at compile time. For example:

struct Door<State>(String, PhantomData<State>);

struct Open;
struct Closed;

fn close(door: Door<Open>) -> Door<Closed> {
    let name = door.0;
	println!("The door '{}' is closed.", name);
	Door::<Closed>(name, PhantomData)
}

fn open(door: Door<Closed>) -> Door<Open> {
    let name = door.0;
	println!("The door '{}' is open.", name);
	Door::<Open>(name, PhantomData)
}

/*
fn do_not_compile() {
	// Attempting to close a closed door is prevented statically:
	let _ = close(Door::<Closed>("front", PhantomData)); // error: mismatched types: expected `Door<Open>`, found `Door<Closed>`
}
*/

#[cfg(not(test))]
fn main() {
	let opened_front_door = Door::<Open>("front".to_string(), PhantomData);
	let closed_front_door = close(opened_front_door);
	let _ = open(closed_front_door);
}

