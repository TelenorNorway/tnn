use crate::util::parent_command::{CommandHandlerReturnType, ParentCommand};
use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};

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
