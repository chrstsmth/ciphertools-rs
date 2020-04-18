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
	type IntoBruteForceIter: Iterator<Item = Self>;

	fn brute_force_iterator() -> Self::BruteForceIter;
	fn into_brute_force_iterator(self) -> Self::IntoBruteForceIter;
}

pub trait IntoMutationIterator: Key {
	type MutationIter: Iterator<Item = Self>;

	fn into_mutation_iterator(self) -> Self::MutationIter;
}

pub trait IntoRandomIterator: Key {
	type RandomIter: Iterator<Item = Self>;
	type Constraint;

	fn into_random_iterator(constraint: Self::Constraint) -> Self::RandomIter;
}
