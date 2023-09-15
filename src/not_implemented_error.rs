use thiserror::Error;

#[derive(Error, Debug)]
#[error("Not yet implemented: {0}")]
pub struct NotImplementedError<'a>(pub &'a str);
