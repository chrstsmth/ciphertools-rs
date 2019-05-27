pub mod vigenere;
pub mod caesar;

use crate::key::*;
use crate::candidate::*;
use std::str::Chars;

pub trait Cipher: Clone + Eq + Ord {
	const NAME: &'static str;
	type Key: Key;

	fn encipher(plaintext: &str, k: &Self::Key) -> String;
	fn decipher(ciphertext: &str, k: &Self::Key) -> String;
}

pub trait DictionaryAttack<S,M,E,J>: Cipher where
	S: Iterator<Item = Self::Key>,
	M: FnMut(&Candidate<Self>),
	E: Fn() -> bool,
	J: Fn(Chars) -> u32,
{
	fn dictionary_attack(ciphertext: &str, dict: S, score: J, candidates: M, exit: E);
}

pub trait BruteForce<S,M,E,J>: DictionaryAttack<S,M,E,J> where
	S: Iterator<Item = Self::Key>,
	M: FnMut(&Candidate<Self>),
	E: Fn() -> bool,
	J: Fn(Chars) -> u32,
{
	type BruteForceKey: Key + IntoBruteForceIterator;

	fn brute_force(ciphertext: &str, score: J, candidates: M, exit: E);
	fn brute_force_from(ciphertext: &str, start: Self::BruteForceKey, score: J, candidates: M, exit: E);
	fn brute_force_between(ciphertext: &str, start: Self::BruteForceKey, end: Self::BruteForceKey, score: J, candidates: M, exit: E);
}

pub trait HillClimb<S,M,E,J>: DictionaryAttack<S,M,E,J> where
	S: Iterator<Item = Self::Key>,
	M: FnMut(&Candidate<Self>),
	E: Fn() -> bool,
	J: Fn(Chars) -> u32,
{
	type MutationKey: Key + IntoMutationIterator;

	fn hill_climb(ciphertext: &str, dict: S, score: J, candidates: M, exit: E);
}
