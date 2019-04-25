use std::convert::TryFrom;
use crate::try_from_err::*;

pub struct CaesarKey(pub char);

impl TryFrom<&str> for CaesarKey {
	type Error = TryFromCharError;

	fn try_from(key: &str) -> Result<CaesarKey, TryFromCharError>
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
