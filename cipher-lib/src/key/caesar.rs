use std::convert::TryFrom;
use std::fmt;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::caesar::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CaesarKey(pub char);

impl Key for CaesarKey {
	type Cipher = Caesar;
}

impl TryFrom<String> for CaesarKey {
	type Error = TryFromCharError;

	fn try_from(key: String) -> Result<CaesarKey, TryFromCharError>
	{
		let mut chars = key.chars();

		match chars.next() {
			Some(first) => {
				match chars.next() {
					None => Ok(CaesarKey(first)),
					_ => Err(TryFromCharError),
				}
			}
			_ => Err(TryFromCharError),
		}
	}
}

impl From<char> for CaesarKey{
	fn from(key: char) -> CaesarKey
	{
		CaesarKey(key)
	}
}

impl fmt::Display for CaesarKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
