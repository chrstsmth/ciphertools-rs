use std::convert::TryFrom;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::caesar::*;

pub struct CaesarKey(pub char);

impl Key for CaesarKey {
	type Cipher = Caesar;
}

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
