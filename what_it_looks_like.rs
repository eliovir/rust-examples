//! Example given on the homepage (2013-10-21) <http://www.rust-lang.org/>
//!
//! Tested with rust-1.3.0
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
use std::thread;

fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    let nums = [1usize, 2];
    let noms = ["Tim", "Eston", "Aaron", "Ben"];
    let odds = nums.iter().map(|&x| x * 2 - 1);

    for num in odds {
        children.push(thread::spawn(move || {
            println!("{} says hello from a lightweight thread!", noms.get(num).unwrap());
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
         let _ = child.join();
    }
}
