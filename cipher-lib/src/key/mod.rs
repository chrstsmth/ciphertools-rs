pub mod vigenere;
pub mod caesar;

use std::convert::TryFrom;
use crate::cipher::*;
use std::fmt;

pub trait Key: TryFrom<String> + fmt::Display + Eq + Ord {
	type Cipher: Cipher;
}

pub trait IntoBruteForceIterator: Key {
	type BruteForceIter: Iterator;

	const START: Self::BruteForceIter;
	fn into_brute_force_iterator(self) -> Self::BruteForceIter;
}

pub trait IntoMutationIterator: Key {
	type MutationIter: Iterator;

	fn into_mutation_iterator() -> Self::MutationIter;
}

