pub mod any_key;
pub mod vigenere;
pub mod caesar;

use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;
use any_key::*;

pub trait Key: Into<AnyKey> + TryFrom<AnyKey> + FromStr + fmt::Display + Clone + Eq + Ord { }

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

	fn random_iterator(constraint: Self::Constraint) -> Self::RandomIter;
}
