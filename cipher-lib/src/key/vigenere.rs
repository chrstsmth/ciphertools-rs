use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use crate::try_from_err::*;
use crate::key::*;
use crate::cipher::vigenere::*;
use crate::pallet::alph::*;

use rand::seq::SliceRandom;

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct VigenereKey(pub Vec<Alph>);

pub struct VigenereKeyBruteForceIterator {
	it: VigenereKey,
}

pub struct VegenereKeyMutationIterator {
	start: VigenereKey,
	index: usize,
	increment: usize,
}

pub struct VegenereKeyRandomIterator {
	lengths: Vec<usize>,
}

impl IntoRandomIterator<Vec<usize>> for VigenereKey {
	type RandomIter = VegenereKeyRandomIterator;

	fn into_random_iterator(constraint: Vec<usize>) -> VegenereKeyRandomIterator
	{
		VegenereKeyRandomIterator {
			lengths: constraint,
		}
	}
}

impl Iterator for VegenereKeyRandomIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {

		let key_len = *self.lengths.choose(&mut rand::thread_rng()).unwrap();
		let mut key = Vec::with_capacity(key_len);
		for _ in 0..key_len {
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
	type BruteForceIter = VigenereKeyBruteForceIterator;

	fn start() -> Self::BruteForceIter {
		VigenereKeyBruteForceIterator {
			it: VigenereKey(vec![Alph::A]),
		}
	}

	fn into_brute_force_iterator(self) -> Self::BruteForceIter {
		VigenereKeyBruteForceIterator {
			it: self,
		}
	}
}

impl Iterator for VigenereKeyBruteForceIterator {
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

impl Ord for VigenereKey {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.0.len() < other.0.len() {
			Ordering::Less
		} else if self.0.len() > other.0.len() {
			Ordering::Greater
		} else {
			self.0.cmp(&other.0)
		}
	}
}
