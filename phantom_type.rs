//!
//! http://static.rust-lang.org/doc/master/complement-cheatsheet.html#how-do-i-express-phantom-types?

// Phantom types are useful for enforcing state at compile time. For example:

struct Door<State>(~str);

struct Open;
struct Closed;

fn close(Door(name): Door<Open>) -> Door<Closed> {
	println!("The door '{}' is closed.", name);
	Door::<Closed>(name)
}

fn open(Door(name): Door<Closed>) -> Door<Open> {
	println!("The door '{}' is open.", name);
	Door::<Open>(name)
}

/*
fn do_not_compile() {
	// Attempting to close a closed door is prevented statically:
	let _ = close(Door::<Closed>(~"front")); // error: mismatched types: expected `main::Door<main::Open>` but found `main::Door<main::Closed>`
}
*/

#[cfg(not(test))]
fn main() {
	let opened_front_door = Door::<Open>("front".to_owned());
	let closed_front_door = close(opened_front_door);
	let _ = open(closed_front_door);
}

