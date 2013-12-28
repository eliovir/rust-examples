#[crate_id="decorator#1.0"];
#[crate_type = "bin"];
#[license = "MIT"];
#[desc = "Implementation of design pattern Decorator" ];
#[comment = "Implement in Rust the design pattern found in Wikipedia (PHP example)."];
/**
 * Design pattern: Decorator
 *
 * Tested with rust-0.9-pre
 *
 * @see http://fr.wikipedia.org/wiki/D%C3%A9corateur_%28patron_de_conception%29#Exemple_en_PHP
 * @author Eliovir <http://github.com/~eliovir>
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 * @since 2013-11-06
 */
struct Message {
	message: ~str,
}

trait Printable {
	fn print(&self) -> ~str;
}
impl Printable for Message {
	fn print(&self) -> ~str {
		self.message.clone()
	}
}

struct UnderlinedMessage<T> {
	decorated: T,
}

// 17.2 Declaring and implementing traits
// http://static.rust-lang.org/doc/master/tutorial.html#declaring-and-implementing-traits
impl<T:Printable> Printable for UnderlinedMessage<T> {
	fn print(&self) -> ~str {
		// http://static.rust-lang.org/doc/master/std/str/trait.StrSlice.html#tymethod.char_len
		let message = self.decorated.print();
		message + "\n" + ("=".repeat(message.char_len()))
	}
}

struct IndentedMessage<T> {
	decorated: T,
}

impl<T:Printable> Printable for IndentedMessage<T> {
	fn print(&self) -> ~str {
		let message = self.decorated.print();
		"    " + message.replace("\n", "\n    ")
	}
}

fn main() {
	let message = Message{message: ~"The first message is underlined then intended."};
	let underlined = UnderlinedMessage{decorated: message};
	let indented = IndentedMessage{decorated: underlined};
	println(indented.print());
	
	let message2 = Message{message: ~"The second message is indented then underlined."};
	let indented2 = IndentedMessage{decorated: message2};
	let underlined2 = UnderlinedMessage{decorated: indented2};
	println(underlined2.print());
}
