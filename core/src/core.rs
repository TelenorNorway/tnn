use clap::Command;

pub struct Core {
	command: Command,
}

impl Core {
	pub fn new(command: Command) -> Self {
		Self { command }
	}

	pub fn add_command(mut self) -> Self {
		self.command = self.command.subcommand(Command::new("hello"));
		self
	}

	pub fn finish(self) -> Command {
		self.command
	}
}
