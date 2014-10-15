#![crate_name="decorator"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Implementation of design pattern Decorator" ]
#![comment = "Implement in Rust the design pattern found in Wikipedia (PHP example)."]
//! Design pattern: Decorator
//!
//! Tested with rust-0.12
//!
//! @see http://fr.wikipedia.org/wiki/D%C3%A9corateur_%28patron_de_conception%29#Exemple_en_PHP
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//! @since 2013-11-06

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
		message.push_str("=".repeat(length).as_slice());
		message
	}
}

struct IndentedMessage<T> {
	decorated: T,
}

impl<T:Printable> Printable for IndentedMessage<T> {
	fn print(&self) -> String {
		let mut message = self.decorated.print();
		message.push_str("    ");
		message.replace("\n", "\n    ")
	}
}

fn main() {
	let message = Message{message: "The first message is underlined then intended.".to_string()};
	let underlined = UnderlinedMessage{decorated: message};
	let indented = IndentedMessage{decorated: underlined};
	println!("{}", indented.print());

	let message2 = Message{message: "The second message is indented then underlined.".to_string()};
	let indented2 = IndentedMessage{decorated: message2};
	let underlined2 = UnderlinedMessage{decorated: indented2};
	println!("{}", underlined2.print());
}
