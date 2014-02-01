/**
 * 5.1 Structs
 * http://static.rust-lang.org/doc/master/tutorial.html#structs
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
/*
 * Rust struct types must be declared before they are used using the struct syntax.
 */
struct Point {
	x: f64,
	y: f64
}
fn main() {
	let mut mypoint = Point { x: 1.0, y: 1.0 };
	let origin = Point { x: 0.0, y: 0.0 };

	println!("origin = {:?}", origin);

	mypoint.y += 1.0; // mypoint is mutable, and its fields as well
	//origin.y += 1.0; // ERROR: assigning to immutable field
	
	// `match` patterns destructure structs.
	match mypoint {
		Point { x: 0.0, y: yy } => println!("{}", yy),
		Point { x: xx,  y: yy } => println!("{} {}", xx, yy),
	}

	// When you are not interested in all the fields of a struct,
	// a struct pattern may end with `, _` 
	// (as in `Name { field1, _ }`) to indicate that you're
	// ignoring all other fields.
	// Additionally, struct fields have a shorthand matching form 
	// that simply reuses the field name as the binding name.
	match mypoint {
		Point { x, .. } => println!("{}", x),
	}
}
