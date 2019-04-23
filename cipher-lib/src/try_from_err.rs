use std::error;
use std::fmt;

#[derive(Debug,Clone)]
pub struct TryFromCharError;

#[derive(Debug,Clone)]
pub struct TryFromIntError;

#[derive(Debug,Clone)]
pub struct TryFromStringError;

impl error::Error for TryFromCharError {
	fn description(&self) -> &str {
		"no conversion available"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for TryFromCharError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "no conversion available")
	}
}

impl error::Error for TryFromIntError {
	fn description(&self) -> &str {
		"no conversion available"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for TryFromIntError  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "no conversion available")
	}
}
