use std::str::Chars;

use crate::cipher::*;
use crate::cipher::vigenere::*;
use crate::key::caesar::*;
use crate::key::vigenere::*;
use crate::candidate::*;
use cipher_derive::*;

#[cfg(test)]
mod tests;

#[derive(DictionaryAttack, BruteForce, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Caesar;

impl Cipher for Caesar {
	const NAME: &'static str = "caesar";
	type Key = CaesarKey;

	fn encipher(plaintext: &str, key: &Self::Key) -> String
	{
		Vigenere::encipher(plaintext, &VigenereKey::from(VigenereKey(vec!(key.0))))
	}
	fn decipher(ciphertext: &str, key: &Self::Key) -> String
	{
		Vigenere::decipher(ciphertext, &VigenereKey::from(VigenereKey(vec!(key.0))))
	}
}
