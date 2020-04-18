use std::str::Chars;

use crate::cipher::*;
use crate::cipher::vigenere::*;
use crate::key::caesar::*;
use crate::key::vigenere::*;
use crate::candidate::*;
use cipher_derive::*;
use crate::alphabet::latin::*;

#[cfg(test)]
mod tests;

#[derive(DictionaryAttack, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Caesar;

impl Cipher for Caesar {
	const NAME: &'static str = "caesar";
	type Key = CaesarKey;

	fn encipher(plaintext: &str, key: &Self::Key) -> String
	{
		Vigenere::encipher(plaintext, &VigenereKey::from([Latin::from(key.clone())].as_ref()))
	}
	fn decipher(ciphertext: &str, key: &Self::Key) -> String
	{
		Vigenere::decipher(ciphertext, &VigenereKey::from([Latin::from(key.clone())].as_ref()))
	}
}
