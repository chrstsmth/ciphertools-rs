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

pub trait DictionaryAttack<Dict,Can,Exit,Score>: Cipher where
	Dict: Iterator<Item = Self::Key>,
	Can: FnMut(&Candidate<Self>),
	Exit: Fn() -> bool,
	Score: Fn(Chars) -> u32,
{
	fn dictionary_attack(ciphertext: &str, dict: Dict, score: Score, candidates: Can, exit: Exit);
}

pub trait BruteForce<Dict,Can,Exit,Score>: DictionaryAttack<Dict,Can,Exit,Score> where
	Dict: Iterator<Item = Self::Key>,
	Can: FnMut(&Candidate<Self>),
	Exit: Fn() -> bool,
	Score: Fn(Chars) -> u32,
{
	type BruteForceKey: Key + IntoBruteForceIterator;

	fn brute_force(ciphertext: &str, score: Score, candidates: Can, exit: Exit);
	fn brute_force_from(ciphertext: &str, start: Self::BruteForceKey, score: Score, candidates: Can, exit: Exit);
	fn brute_force_between(ciphertext: &str, start: Self::BruteForceKey, end: Self::BruteForceKey, score: Score, candidates: Can, exit: Exit);
}

pub trait HillClimb<Dict,Can,Exit,Score>: DictionaryAttack<Dict,Can,Exit,Score> where
	Dict: Iterator<Item = Self::Key>,
	Can: FnMut(&Candidate<Self>),
	Exit: Fn() -> bool,
	Score: Fn(Chars) -> u32,
{
	type MutationKey: Key + IntoMutationIterator;

	fn hill_climb(ciphertext: &str, dict: Dict, score: Score, candidates: Can, exit: Exit);
}
