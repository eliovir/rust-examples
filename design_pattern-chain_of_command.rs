#![crate_id="design_pattern-chain_of_command#1.0"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from PHP code of
//! http://codersview.blogspot.fr/2009/05/chain-of-command-pattern-using-php.html
//!
//! Tested with rust-0.11-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-20

trait Command {
	fn onCommand(&self, name: &str, args: &[~str]);
}

struct CommandChain {
	commands: Vec<Box<Command>>,
}
impl CommandChain {
	fn new() -> CommandChain {
		CommandChain{commands: Vec::new()}
	}
	fn addCommand(&mut self, command: Box<Command>) {
		self.commands.push(command);
	}
	fn runCommand(&self, name: &str, args: &[~str]) {
		for command in self.commands.iter() {
			command.onCommand(name, args);
		}
	}
}

struct UserCommand;
impl UserCommand {
	fn new() -> UserCommand {
		UserCommand
	}
}
impl Command for UserCommand {
	#[allow(unused_variable)]
	fn onCommand(&self, name: &str, args: &[~str]) {
		if name == "addUser" {
			println!("UserCommand handling '{}'.", name);
		}
	}
}

struct MailCommand;
impl MailCommand {
	fn new() -> MailCommand {
		MailCommand
	}
}
impl Command for MailCommand {
	fn onCommand(&self, name: &str, args: &[~str]) {
		if name == "addUser" {
			println!("MailCommand handling '{}' with args {}.", name, args);
		} else if name == "mail" {
			println!("MailCommand handling '{}' with args {}.", name, args);
		}
	}
}
fn main() {
	let mut cc = CommandChain::new();
	cc.addCommand(box UserCommand::new());
	cc.addCommand(box MailCommand::new());
	cc.runCommand("addUser", &["Toto".to_owned(), "users".to_owned()]);
	cc.runCommand("mail", &["Sender".to_owned(), "Subject".to_owned()]);
}
