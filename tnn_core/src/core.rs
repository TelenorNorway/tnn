use clap::{CommandFactory, FromArgMatches};
use util_tnn_parent_command::{CommandHandlerReturnType, ParentCommand};

pub struct Core {
	command: ParentCommand,
}

impl Core {
	pub fn new(command: ParentCommand) -> Self {
		Self { command }
	}

	pub fn add_command<Command: CommandFactory + FromArgMatches + 'static>(
		mut self,
		handler: &'static impl Fn(&Command) -> CommandHandlerReturnType,
	) -> Self {
		self.command = self.command.add_command::<Command>(handler).unwrap();
		self
	}

	pub fn finish(self) -> ParentCommand {
		self.command
	}
}
