use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use util_tnn_parent_command::{CommandHandlerReturnType, ParentCommand};

pub struct Core {
	command: ParentCommand,
}

impl<'a> Core {
	pub fn new(command: ParentCommand) -> Self {
		Self { command }
	}

	pub fn add_command<Command: CommandFactory + FromArgMatches + 'static>(
		mut self,
		handler: Box<dyn Fn(Command) -> CommandHandlerReturnType>,
	) -> Result<Self> {
		self.command = self.command.add_command::<Command>(handler)?;
		Ok(self)
	}

	pub fn finish(self) -> ParentCommand {
		self.command
	}
}
