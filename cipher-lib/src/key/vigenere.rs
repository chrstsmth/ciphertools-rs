use std::convert::TryFrom;
use std::fmt;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::vigenere::*;
use crate::pallet::alph::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VigenereKey(pub Vec<Alph>);

pub struct VegenereKeyIterator {
	it: VigenereKey,
}

impl Key for VigenereKey {
	type Cipher = Vigenere;
}

impl FromStr for VigenereKey {
	type Err = TryFromStringError;

	fn from_str(s: &str) -> Result<VigenereKey, TryFromStringError>
	{
		let mut vigenere_key = VigenereKey(Vec::<Alph>::with_capacity(s.len()));
		for c in s.chars() {
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

impl IntoBruteForceIterator for VigenereKey {
	type BruteForceIter = VegenereKeyIterator;

	fn start() -> Self::BruteForceIter {
		VegenereKeyIterator {
			it: VigenereKey(vec![Alph::A]),
		}
	}

	fn into_brute_force_iterator(self) -> Self::BruteForceIter {
		VegenereKeyIterator {
			it: self.clone(),
		}
	}
}

impl Iterator for VegenereKeyIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {
		let r = self.it.clone();
		let mut overflow = true;

		for a in &mut self.it.0 {
			let mut i: usize = usize::from(*a);
			i = (i + 1) % Alph::SIZE;
			*a = Alph::try_from(i).unwrap();

			if i != 0 {
				overflow = false;
				break;
			}
		}

		if overflow {
			self.it.0.push(Alph::A);
		}
		Some(r)
	}
}

