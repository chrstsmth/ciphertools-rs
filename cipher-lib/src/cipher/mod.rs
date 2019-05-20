pub mod vigenere;
pub mod caesar;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::key::*;
use crate::candidate::*;
use crate::language_model::*;

pub trait Cipher {
	const NAME: &'static str;
	type Key: Key;

	fn encipher(plaintext: &String, k: &Self::Key) -> String;
	fn decipher(ciphertext: &String, k: &Self::Key) -> String;
}

pub trait DictionaryAttack<S,M>: Cipher where
	S: Iterator<Item = Self::Key>,
	M: Model<Self::Key>,
{
	fn dictionary_attack(ciphertext: &String, dict: S, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>);
}

pub trait BruteForce<S,M>: DictionaryAttack<S,M> where
	S: Iterator<Item = Self::Key>,
	M: Model<Self::Key>,
{
	type BruteForceKey: Key + IntoBruteForceIterator;

	fn brute_force(ciphertext: &String, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>);
	fn brute_force_from(ciphertext: &String, start: Self::BruteForceKey, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>);
	fn brute_force_between(ciphertext: &String, start: Self::BruteForceKey, end: Self::BruteForceKey, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>);
}
