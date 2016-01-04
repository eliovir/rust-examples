//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-1.3.O
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-20

//! Each action is encapsulated into a struct with the trait Command
//! where only the method `execute()` is run.
trait Command {
	fn execute(&self);
}

// Use a Null struct to initialize the remote control.
struct NullCommand;
impl NullCommand {
	fn new() -> NullCommand {
		NullCommand
	}
}
impl Command for NullCommand {
	fn execute(&self) {
		println!("Nothing to do!");
	}
}

// The object to handle: a light
#[derive(Copy, Clone)]
struct Light;
impl Light {
	fn new() -> Light {
		Light
	}
	fn on(&self) {
		println!("Light is on");
	}
	fn off(&self) {
		println!("Light is off");
	}
}

// The first command on the object: light on
struct LightOnCommand {
	light: Light,
}
impl LightOnCommand {
	fn new(light: Light) -> LightOnCommand {
		LightOnCommand { light: light }
	}
}
impl Command for LightOnCommand {
	fn execute(&self) {
		self.light.on();
	}
}

// The second command on the object: light off
struct LightOffCommand {
	light: Light,
}
impl LightOffCommand {
	fn new(light: Light) -> LightOffCommand {
		LightOffCommand { light: light }
	}
}
impl Command for LightOffCommand {
	fn execute(&self) {
		self.light.off();
	}
}

// The command will be launched by a remote control.
struct SimpleRemoteControl<'a> {
	command: Box<Command + 'a>,
}
impl<'a> SimpleRemoteControl<'a> {
	fn new() -> SimpleRemoteControl<'a> {
		SimpleRemoteControl { command: Box::new(NullCommand::new()) }
	}
	fn set_command(&mut self, cmd: Box<Command + 'a>) {
		self.command = cmd;
	}
	fn button_was_pressed(&self) {
		self.command.execute();
	}
}

fn main() {
	let mut remote = SimpleRemoteControl::new();
	let light = Light::new();
	let light_on = LightOnCommand::new(light);
	let light_off = LightOffCommand::new(light);

	remote.button_was_pressed();
	remote.set_command(Box::new(light_on));
	remote.button_was_pressed();
	remote.set_command(Box::new(light_off));
	remote.button_was_pressed();
}
