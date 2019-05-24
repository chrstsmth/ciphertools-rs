pub mod vigenere;
pub mod caesar;

use std::str::FromStr;
use crate::cipher::*;
use std::fmt;

pub trait Key: FromStr + fmt::Display + Clone + Eq + Ord {
	type Cipher: Cipher;
}

pub trait IntoBruteForceIterator: Key {
	type BruteForceIter: Iterator;

	fn start() -> Self::BruteForceIter;
	fn into_brute_force_iterator(self) -> Self::BruteForceIter;
}

pub trait IntoMutationIterator: Key {
	type MutationIter: Iterator;

	fn into_mutation_iterator() -> Self::MutationIter;
}

