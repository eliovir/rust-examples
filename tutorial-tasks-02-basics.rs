/**
 * Rust Tasks and Communication Tutorial - 2 Basics
 * http://static.rust-lang.org/doc/0.7/tutorial-tasks.html#basics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::int;

fn generate_task_number() -> int {
	10
}
fn main() {
	// Print something profound in a different task using a named function
	fn print_message() { println("I am running in a different task!"); }
	spawn(print_message);

	// Print something more profound in a different task using a lambda expression
	spawn( || println("I am also running in a different task!") );

	// The canonical way to spawn is using `do` notation
	do spawn {
		    println("I too am running in a different task!");
	}

	/*
	 */
	// Generate some state locally
	let child_task_number_10 = generate_task_number();

	do spawn {
		   // Capture it in the remote task
		   println(fmt!("I am child number %d", child_task_number_10));
	}

	/*
	 */
	for int::range(0, 20) |child_task_number| {
		do spawn {
			print(fmt!("I am child number %d\n", child_task_number));
		}
	}

}
