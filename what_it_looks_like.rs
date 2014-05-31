//!
//! Example given on the homepage (2013-10-21) http://www.rust-lang.org/
//!
//! Tested with rust-0.11-pre
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
fn main() {
    let nums = [1u, 2];
    let noms = ["Tim", "Eston", "Aaron", "Ben"];

    let mut odds = nums.iter().map(|&x| x * 2 - 1);

    for num in odds {
        spawn(proc()
            println!("{:s} says hello from a lightweight thread!", noms[num])
        );
    }
}
