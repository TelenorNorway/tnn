use anyhow::Result;
use clap::Args;

use crate::not_implemented_error::NotImplementedError;

#[derive(Args)]
pub struct TnnOpShellArgs {}

/// Connect to the Nova Operator Server with SSH
pub async fn op_shell_command(_args: &TnnOpShellArgs) -> Result<()> {
	Err(NotImplementedError("James Bradlee, implement this").into())
}
