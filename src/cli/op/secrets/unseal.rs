use anyhow::Result;
use clap::Args;

use crate::not_implemented_error::NotImplementedError;

#[derive(Args)]
pub struct TnnOpSecretsUnsealArgs {}

/// Connect to the Nova Operator Server with SSH
pub async fn op_secrets_unseal_command(_args: &TnnOpSecretsUnsealArgs) -> Result<()> {
	Err(NotImplementedError("James Bradlee, implement this").into())
}
