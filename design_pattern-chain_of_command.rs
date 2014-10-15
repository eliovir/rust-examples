#![crate_name="design_pattern-chain_of_command"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from PHP code of
//! http://codersview.blogspot.fr/2009/05/chain-of-command-pattern-using-php.html
//!
//! Tested with rust-0.12
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-20

trait Command {
	fn on_command(&self, name: &str, args: &[&str]);
}

struct CommandChain<'a> {
	commands: Vec<Box<Command + 'a>>,
}
impl<'a> CommandChain<'a> {
	fn new() -> CommandChain<'a> {
		CommandChain{commands: Vec::new()}
	}
	fn add_command(&mut self, command: Box<Command + 'a>) {
		self.commands.push(command);
	}
	fn run_command(&self, name: &str, args: &[&str]) {
		for command in self.commands.iter() {
			command.on_command(name, args);
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
	fn on_command(&self, name: &str, args: &[&str]) {
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
	fn on_command(&self, name: &str, args: &[&str]) {
		if name == "addUser" {
			println!("MailCommand handling '{}' with args {}.", name, args);
		} else if name == "mail" {
			println!("MailCommand handling '{}' with args {}.", name, args);
		}
	}
}
fn main() {
	let mut cc = CommandChain::new();
	cc.add_command(box UserCommand::new());
	cc.add_command(box MailCommand::new());
	cc.run_command("addUser", &["Toto", "users"]);
	cc.run_command("mail", &["Sender", "Subject"]);
}
