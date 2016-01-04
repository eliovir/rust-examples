//! Example given on the homepage (2014-04-03) http://www.rust-lang.org/
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
fn main() {
    // A simple integer calculator:
    // `+` or `-` means add/sub by 1
    // `*` or `/` means mul/div by 2

    let program = "+ + * - /";
    let mut accumulator = 0i32;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* ignore everything else */ }
        }
    }

    println!("The program \"{}\" calculates the value {}",
             program, accumulator);
}
