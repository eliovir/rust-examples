//! Design pattern: Decorator
//!
//! Implement in Rust the design pattern found in Wikipedia (PHP example).
//!
//! Tested with rust-1.3.0
//!
//! @see http://fr.wikipedia.org/wiki/D%C3%A9corateur_%28patron_de_conception%29#Exemple_en_PHP
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//! @since 2013-11-06
use std::iter::repeat;

struct Message {
	message: String,
}

trait Printable {
	fn print(&self) -> String;
}
impl Printable for Message {
	fn print(&self) -> String {
		self.message.clone()
	}
}

struct UnderlinedMessage<T> {
	decorated: T,
}

// 17.2 Declaring and implementing traits
// http://doc.rust-lang.org/tutorial.html#declaring-and-implementing-traits
impl<T:Printable> Printable for UnderlinedMessage<T> {
	fn print(&self) -> String {
		// http://doc.rust-lang.org/std/str/trait.StrSlice.html#tymethod.char_len
		let mut message = self.decorated.print();
		let length = message.len();
		message.push_str("\n");
		message.push_str(&repeat("=").take(length).collect::<String>());
		message
	}
}

struct IndentedMessage<T> {
	decorated: T,
}

impl<T:Printable> Printable for IndentedMessage<T> {
	fn print(&self) -> String {
		let message = "    ".to_string() + & self.decorated.print();
		message.replace("\n", "\n    ")
	}
}

fn main() {
	let message = Message{message: "Underlined message.".to_string()};
	let underlined = UnderlinedMessage{decorated: message};
	println!("{}", underlined.print());

	let message = Message{message: "Indented message.".to_string()};
	let indented = IndentedMessage{decorated: message};
	println!("{}", indented.print());

	let message = Message{message: "The first message is underlined then intended.".to_string()};
	let underlined = UnderlinedMessage{decorated: message};
	let indented = IndentedMessage{decorated: underlined};
	println!("{}", indented.print());

	let message2 = Message{message: "The second message is indented then underlined.".to_string()};
	let indented2 = IndentedMessage{decorated: message2};
	let underlined2 = UnderlinedMessage{decorated: indented2};
	println!("{}", underlined2.print());
}
