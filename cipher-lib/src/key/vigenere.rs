use std::convert::TryFrom;
use std::fmt;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::vigenere::*;
use crate::pallet::alph::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VigenereKey(pub Vec<Alph>);

impl Key for VigenereKey {
	type Cipher = Vigenere;
}

impl TryFrom<String> for VigenereKey {
	type Error = TryFromStringError;

	fn try_from(key: String) -> Result<VigenereKey, TryFromStringError>
	{
		let mut vigenere_key = VigenereKey(Vec::<Alph>::with_capacity(key.len()));
		for c in key.chars() {
			let alph = match Alph::try_from(c) {
				Err(_) => return Err(TryFromStringError),
				Ok(alph) => alph
			};
			vigenere_key.0.push(alph);
		}
		Ok(vigenere_key)
	}
}

impl fmt::Display for VigenereKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::with_capacity(self.0.len());

		for a in &self.0 {
			s.push(char::from(*a));
		}

		write!(f, "{}", s)
	}
}

