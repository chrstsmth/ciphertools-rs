use std::convert::TryFrom;
use std::fmt;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::caesar::*;
use crate::pallet::alph::*;
use crate::pallet::cipher_char::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaesarKey(pub Alph);

pub struct CaesarKeyIterator {
	start: CaesarKey,
	i: u32,
}

impl Key for CaesarKey {
	type Cipher = Caesar;
}

impl FromStr for CaesarKey {
	type Err = TryFromStringError;

	fn from_str(s: &str) -> Result<CaesarKey, TryFromStringError>
	{
		let mut chars = s.chars();

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

impl IntoBruteForceIterator for CaesarKey {
	type BruteForceIter = CaesarKeyIterator;

	fn start() -> Self::BruteForceIter {
		CaesarKeyIterator {
			start: CaesarKey(Alph::A),
			i: 0,
		}
	}

	fn into_brute_force_iterator(self) -> Self::BruteForceIter {
		CaesarKeyIterator {
			start: self.clone(),
			i: 0,
		}
	}
}

impl Iterator for CaesarKeyIterator {
	type Item = CaesarKey;

	fn next(&mut self) -> Option<Self::Item> {
		let r = if self.i == Alph::LENGTH {
			None
		} else {
			let a = CipherChar::from(Alph::try_from(self.i).unwrap());
			let b = CipherChar::from(self.start.0);
			Some(CaesarKey(Alph::try_from((a + b).0).unwrap()))
		};
		self.i += 1;
		r
	}
}

impl fmt::Display for CaesarKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
