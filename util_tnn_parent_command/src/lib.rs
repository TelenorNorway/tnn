use anyhow::Result;
use clap::{ArgMatches, Command, CommandFactory, FromArgMatches};
use std::{collections::HashMap, future::Future, pin::Pin};
use thiserror::Error;

pub type CommandHandlerReturnType = Pin<Box<dyn Future<Output = Result<()>>>>;

pub struct ParentCommand {
	inner: Command,
	wrapped_command_handlers: HashMap<String, Box<dyn Fn(&ArgMatches) -> CommandHandlerReturnType>>,
}

impl ParentCommand {
	pub fn new(command: Command) -> Self {
		Self {
			inner: command,
			wrapped_command_handlers: HashMap::new(),
		}
	}

	pub fn add_command<Command: CommandFactory + FromArgMatches + 'static>(
		mut self,
		handler: &'static impl Fn(&Command) -> CommandHandlerReturnType,
	) -> Result<Self> {
		let subcommand = Command::command();
		let subcommand_name: String = subcommand.get_name().to_string();

		if self.wrapped_command_handlers.contains_key(&subcommand_name) {
			return Err(
				CommandExistsError(self.inner.get_name().to_string(), subcommand.get_name().to_string()).into(),
			);
		}

		// todo(James Bradlee): Assert that command aliases does not exist.

		self.wrapped_command_handlers.insert(
			subcommand_name,
			Box::new(|matches| {
				handler(&Command::from_arg_matches(matches).expect("Could not derive argument matches"))
			}),
		);

		self.inner = self.inner.subcommand(subcommand);

		Ok(self)
	}

	pub fn build(self) -> (Command, Box<dyn Fn(&ArgMatches) -> CommandHandlerReturnType>) {
		(
			self.inner,
			Box::new(move |matches| {
				if let Some((subcommand_name, subcommand_matches)) = matches.subcommand() {
					if let Some(subcommand_handler) = self.wrapped_command_handlers.get(&subcommand_name.to_string()) {
						return subcommand_handler(subcommand_matches);
					}
				}
				Box::pin(async { Ok(()) })
			}),
		)
	}
}

#[derive(Error, Debug)]
#[error("Command {1} already exists in parent command {0}")]
pub struct CommandExistsError(String, String);
