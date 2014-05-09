/**
 * Program to an 'interface', not an 'implementation'.
 *
 * http://joshldavis.com/2013/07/01/program-to-an-interface-fool/
 *
 * @author: Josh Davis
 * @date: 2013-07-01
 */

/**
 * Imagine you are Guy Montag from the book, Fahrenheit 451 (F451 from here on out).
 * As everyone knows, books in F451 are forbidden.
 * It's the job of firefighters to set them on fire whenever they come across them.
 * Therefore thinking in terms of OOP, a book has a method called `burn()`.
 */
struct Book {
	title: ~str,
	author: ~str,
}

struct Log {
	wood_type: ~str,
}

trait Burns {
	fn burn(&self);
}

impl Burns for Log {
	fn burn(&self) {
		println!("The {} log is burning!", self.wood_type);
	}
}

impl Burns for Book {
	fn burn(&self) {
		println!("The book \"{}\" by {} is burning!", self.title, self.author);
	}
}

struct Incunable(Book);

impl Burns for Incunable {
	fn burn(&self) {
		let book: &Book = match *self {
			Incunable(ref book) => book
		};
		println!("The incunable \"{}\" by {} is burning!", book.title, book.author);
	}
}

/**
 * This is where the power of programming to an interface comes in.
 * Rather than expecting a Book object or a Log object, we just take in any object with any type (we call the type T) that implements the Burns interface.
 */
fn start_fire<T: Burns>(item: T) {
	item.burn();
}

fn main() {
	let lg = Log {
		wood_type: "Oak".to_owned(),
	};
	let book = Book {
		title: "The Brothers Karamazov".to_owned(),
		author: "Fyodor Dostoevsky".to_owned(),
	};
	let nuremberg_chronicle = Book {
		title: "Liber Chronicarum".to_owned(),
		author: "Hartmann Schedel".to_owned(),
	};
	let incunable = Incunable(nuremberg_chronicle);
	// Burn the oak log!
	start_fire(lg);
	
	// Burn the book!
	start_fire(book);
	start_fire(incunable);
}

