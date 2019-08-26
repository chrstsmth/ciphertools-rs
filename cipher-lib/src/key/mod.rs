pub mod vigenere;
pub mod caesar;

use std::str::FromStr;
use crate::cipher::*;
use std::fmt;

pub trait Key: FromStr + fmt::Display + Clone + Eq + Ord {
	type Cipher: Cipher;
}

pub trait IntoBruteForceIterator: Key {
	type BruteForceIter: Iterator<Item = Self>;

	fn start() -> Self::BruteForceIter;
	fn into_brute_force_iterator(self) -> Self::BruteForceIter;
}

pub trait IntoMutationIterator: Key {
	type MutationIter: Iterator<Item = Self>;

	fn into_mutation_iterator(self) -> Self::MutationIter;
}

pub trait IntoRandomIterator: Key {
	type RandomIter: Iterator<Item = Self>;

	fn into_random_iterator() -> Self::RandomIter;
}
