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

pub struct VegenereKeyMutationIterator {
	start: VigenereKey,
	index: usize,
	increment: usize,
}

pub struct VegenereKeyRandomIterator {
	len: usize,
}

impl VigenereKey {
	pub fn into_random_iterator(key_len: usize) -> VegenereKeyRandomIterator
	{
		VegenereKeyRandomIterator {
			len: key_len,
		}
	}
}

impl Iterator for VegenereKeyRandomIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {
		let mut key = Vec::with_capacity(self.len);
		for _ in 0..self.len {
			key.push(rand::random());
		}
		Some(VigenereKey(key))
	}
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
		for a in &self.0 {
			write!(f, "{}", *a)?
		}
		Ok(())
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
			it: self,
		}
	}
}

impl Iterator for VegenereKeyIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {
		let item = self.it.clone();
		let mut overflow = true;

		for a in &mut self.it.0.iter_mut().rev() {
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
		Some(item)
	}
}

impl Iterator for VegenereKeyMutationIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {
		self.increment = (self.increment + 1) % Alph::SIZE;

		if self.increment == 0 {
			self.increment = 1;
			self.index += 1;
			if self.index == self.start.0.len() {
				return None;
			}
		}

		let mut item = self.start.clone();
		item.0[self.index] = item.0[self.index] + Alph::try_from(self.increment).unwrap();
		Some(item)
	}
}

impl IntoMutationIterator for VigenereKey {
	type MutationIter = VegenereKeyMutationIterator;

	fn into_mutation_iterator(self) -> Self::MutationIter {
		VegenereKeyMutationIterator {
			start: self,
			index: 0,
			increment: 0,
		}
	}
}
