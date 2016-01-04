/**
 * Dining Philosophers, a classic concurrency problem.
 *
 * http://doc.rust-lang.org/book/dining-philosophers.html
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
struct Philosopher {
	name: String,
}

impl Philosopher {
	fn new(name: &str) -> Philosopher {
		Philosopher {
			name: name.to_string(),
		}
	}
	fn eat(&self) {
		println!("{} is done eating.", self.name);
	}
}

fn main() {
	let philosophers = vec![
		Philosopher::new("Judith Butler"),
		Philosopher::new("Gilles Deleuze"),
		Philosopher::new("Karl Marx"),
		Philosopher::new("Emma Goldman"),
		Philosopher::new("Michel Foucault"),
	];

	for p in &philosophers {
		p.eat();
	}
}

