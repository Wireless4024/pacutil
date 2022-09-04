use std::fmt::{Debug, Display, Formatter};
use serde::ser::StdError;

#[derive(Debug)]
pub struct SerError(pub String);

impl Display for SerError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl StdError for SerError {}

impl serde::ser::Error for SerError {
	fn custom<T>(msg: T) -> Self where T: Display {
		Self(msg.to_string())
	}
}
