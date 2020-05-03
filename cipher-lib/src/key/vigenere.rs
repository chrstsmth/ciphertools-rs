use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use crate::key::*;
use crate::alphabet::latin::*;

use rand::seq::SliceRandom;

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct VigenereKey(Vec<Latin>);

pub struct VigenereKeyBruteForceIterator {
	it: VigenereKey,
}

pub struct VegenereKeyMutationIterator {
	start: VigenereKey,
	index: usize,
	increment: u32,
}

pub struct VegenereKeyRandomIterator {
	lengths: Vec<usize>,
}

impl VigenereKey {
	pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
		self.into_iter()
	}
}

impl From<VigenereKey> for AnyKey {
	fn from(k: VigenereKey) -> AnyKey {
		AnyKey::Vigenere(k)
	}
}

impl TryFrom<AnyKey> for VigenereKey {
	type Error = &'static str;

	fn try_from(a: AnyKey) -> Result<Self, Self::Error> {
		if let AnyKey::Vigenere(k) = a {
			Ok(k)
		} else {
			Err("try_from::<AnyKey> for CaesarKey")
		}
	}
}

impl From<&[Latin]> for VigenereKey {
	fn from(s: &[Latin]) -> VigenereKey {
		VigenereKey(s.to_vec())
	}
}

impl<'a> IntoIterator for &'a VigenereKey {
	type Item = &'a Latin;
	type IntoIter = std::slice::Iter<'a, Latin>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl<'a> IntoIterator for VigenereKey {
	type Item = Latin;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl IntoRandomIterator for VigenereKey {
	type RandomIter = VegenereKeyRandomIterator;
	type Constraint = Vec<usize>;

	fn random_iterator(constraint: Self::Constraint) -> VegenereKeyRandomIterator {
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

impl Key for VigenereKey { }

impl FromStr for VigenereKey {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<VigenereKey, Self::Err>
	{
		let mut vigenere_key = VigenereKey(Vec::<Latin>::with_capacity(s.len()));
		for c in s.chars() {
			let alph = match Latin::try_from(c) {
				Err(_) => return Err("Failed VigenereKey::FromStr"),
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
	type IntoBruteForceIter = VigenereKeyBruteForceIterator;

	fn brute_force_iterator() -> Self::BruteForceIter {
		VigenereKeyBruteForceIterator {
			it: VigenereKey(vec![Latin::A]),
		}
	}

	fn into_brute_force_iterator(self) -> Self::IntoBruteForceIter {
		VigenereKeyBruteForceIterator {
			it: self,
		}
	}
}

impl Iterator for VigenereKeyBruteForceIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {
		let item = self.it.clone();
		let mut wrap = true;

		for a in &mut self.it.0.iter_mut().rev() {
			let mut i: u32 = u32::from(*a);
			i = (i + 1) % Latin::LENGTH;
			*a = Latin::try_from(i).unwrap();

			if i != 0 {
				wrap = false;
				break;
			}
		}

		if wrap {
			self.it.0.push(Latin::A);
		}
		Some(item)
	}
}

impl Iterator for VegenereKeyMutationIterator {
	type Item = VigenereKey;

	fn next(&mut self) -> Option<Self::Item> {
		self.increment = (self.increment + 1) % Latin::LENGTH;

		if self.increment == 0 {
			self.increment = 1;
			self.index += 1;
			if self.index == self.start.0.len() {
				return None;
			}
		}

		let mut item = self.start.clone();
		item.0[self.index] = item.0[self.index] + Latin::try_from(self.increment).unwrap();
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
