pub mod vigenere;
pub mod caesar;

use crate::key::*;
use crate::candidate::*;
use crate::language_model::*;

pub trait Cipher: Clone + Eq + Ord {
	const NAME: &'static str;
	type Key: Key;

	fn encipher(plaintext: &String, k: &Self::Key) -> String;
	fn decipher(ciphertext: &String, k: &Self::Key) -> String;
}

pub trait DictionaryAttack<S,M,E>: Cipher where
	S: Iterator<Item = Self::Key>,
	M: FnMut(Candidate<Self>),
	E: Fn() -> bool,
{
	fn dictionary_attack(ciphertext: &String, dict: S, lang: LanguageModel, candidates: M, exit: E);
}

pub trait BruteForce<S,M,E>: DictionaryAttack<S,M,E> where
	S: Iterator<Item = Self::Key>,
	M: FnMut(Candidate<Self>),
	E: Fn() -> bool,
{
	type BruteForceKey: Key + IntoBruteForceIterator;

	fn brute_force(ciphertext: &String, lang: LanguageModel, candidates: M, exit: E);
	fn brute_force_from(ciphertext: &String, start: Self::BruteForceKey, lang: LanguageModel, candidates: M, exit: E);
	fn brute_force_between(ciphertext: &String, start: Self::BruteForceKey, end: Self::BruteForceKey, lang: LanguageModel, candidates: M, exit: E);
}
