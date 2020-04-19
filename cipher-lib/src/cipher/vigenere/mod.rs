use crate::cipher::*;
use crate::key::vigenere::*;
use crate::character::caesar_char::*;
use std::iter::Iterator;
use cipher_derive::*;

#[cfg(test)]
mod tests;

#[derive(DictionaryAttack, HillClimb, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vigenere;

impl<'a> Cipher for Vigenere {
	const NAME: &'static str = "vigenere";
	type Key = VigenereKey;
	type Config = ();

	fn encipher(plaintext: &str, key: &Self::Key, _config: &Self::Config) -> String
	{
		let mut ciphertext = String::with_capacity(plaintext.len());
		for (p, k) in plaintext.chars().zip(key.iter().cycle()) {
			ciphertext.push(char::from(CaesarChar::from(p) + *k));
		}
		ciphertext
	}
	fn decipher(ciphertext: &str, key: &Self::Key, _config: &Self::Config) -> String
	{
		let mut plaintext = String::with_capacity(ciphertext.len());
		for (p, k) in ciphertext.chars().zip(key.iter().cycle()) {
			plaintext.push(char::from(CaesarChar::from(p) - *k));
		}
		plaintext
	}
}

