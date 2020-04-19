pub mod vigenere;
pub mod caesar;

use crate::key::*;
use crate::candidate::*;
use std::str::Chars;

pub trait Cipher: Clone + Eq + Ord {
	const NAME: &'static str;
	type Key: Key;
	type Config;

	fn encipher(plaintext: &str, k: &Self::Key, config: &Self::Config) -> String;
	fn decipher(ciphertext: &str, k: &Self::Key, config: &Self::Config) -> String;
}

pub trait DictionaryAttack: Cipher
{
	fn dictionary_attack<Dict,Can,Exit,Score>(ciphertext: &str, dict: Dict, config: &Self::Config, score: Score, candidates: Can, exit: Exit) where
		Dict: Iterator<Item = Self::Key>,
		Can: FnMut(&Candidate<Self>),
		Exit: Fn() -> bool,
		Score: Fn(Chars) -> u32;
}

pub trait HillClimb: DictionaryAttack
{
	type MutationKey: Key + IntoMutationIterator;

	fn hill_climb<Dict,Can,Exit,Score>(ciphertext: &str, dict: Dict, config: &Self::Config, score: Score, candidates: Can, exit: Exit) where
		Dict: Iterator<Item = Self::Key>,
		Can: FnMut(&Candidate<Self>),
		Exit: Fn() -> bool,
		Score: Fn(Chars) -> u32;
}
