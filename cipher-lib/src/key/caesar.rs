use std::convert::TryFrom;
use std::fmt;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::caesar::*;
use crate::pallet::alph::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaesarKey(pub Alph);


impl Key for CaesarKey {
	type Cipher = Caesar;
}

impl TryFrom<String> for CaesarKey {
	type Error = TryFromStringError;

	fn try_from(key: String) -> Result<CaesarKey, TryFromStringError>
	{
		let mut chars = key.chars();

		let first = match chars.next() {
			Some(first) => first,
			_ => return Err(TryFromStringError),
		};

		match chars.next() {
			Some(_) => return Err(TryFromStringError),
			_ => (),
		};

		let alph = match  Alph::try_from(first) {
			Ok(alph) => alph,
			_ => return Err(TryFromStringError),
		};

		Ok(CaesarKey(alph))
	}
}

impl fmt::Display for CaesarKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
