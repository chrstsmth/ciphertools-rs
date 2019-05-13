pub mod vigenere;
pub mod caesar;

use crate::key::*;
use crate::candidate::*;
use crate::language_model::*;

pub trait Cipher {
	type Key: Key;

	fn encipher(plaintext: &String, k: &Self::Key) -> String;
	fn decipher(ciphertext: &String, k: &Self::Key) -> String;
}

pub trait DictionaryAttack<S>: Cipher where
	S: Iterator<Item = Self::Key>,
{
	fn dictionary_attack(ciphertext: &String, dictionary: S, n: usize, lang: LanguageModel) -> Vec<Candidate<Self::Key>>;
}
