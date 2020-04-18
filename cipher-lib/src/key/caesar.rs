use std::convert::TryFrom;
use std::fmt;
use crate::key::*;
use crate::cipher::caesar::*;
use crate::alphabet::latin::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CaesarKey(Latin);

pub struct CaesarKeyIterator<I: Iterator<Item = Latin>>(I);

impl From<Latin> for CaesarKey {
	fn from(c: Latin) -> Self {
		CaesarKey(c)
	}
}

impl From<CaesarKey> for Latin {
	fn from(c: CaesarKey) -> Latin {
		c.0
	}
}

impl Key for CaesarKey {
	type Cipher = Caesar;
}

impl FromStr for CaesarKey {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<CaesarKey, Self::Err>
	{
		let error = "Failed to parse Ceasar Key: Should be one alphabet character";
		let mut chars = s.chars();

		let first = match chars.next() {
			Some(first) => first,
			_ => return Err(error),
		};

		match chars.next() {
			Some(_) => return Err(error),
			_ => (),
		};
		let alph = match Latin::try_from(first) {
			Ok(a) => a,
			_ => return Err(error),
		};

		Ok(CaesarKey(alph))
	}
}

impl IntoBruteForceIterator for CaesarKey
{
	type BruteForceIter = impl Iterator<Item = Self>;
	type IntoBruteForceIter = impl Iterator<Item = Self>;

	fn brute_force_iterator() -> Self::BruteForceIter {
		CaesarKeyIterator(Latin::iter())
	}

	fn into_brute_force_iterator(self) -> Self::IntoBruteForceIter {
		Self::brute_force_iterator().skip(u32::from(self.0) as usize)
	}
}

impl<I> Iterator for CaesarKeyIterator<I>
where
	I : Iterator<Item = Latin>
{
	type Item = CaesarKey;
	fn next(&mut self) -> Option<Self::Item> {
		match self.0.next() {
			Some(l) => Some(CaesarKey(l)),
			None => None
		}
	}

}

impl fmt::Display for CaesarKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
