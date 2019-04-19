use super::*;
use pallet::cipher_char::*;
use std::iter::Iterator;

pub struct Vigenere;

impl Cipher for Vigenere {
	type Key = String;

	fn encipher(plaintext: String, key: Self::Key) -> String
	{
		let mut ciphertext = String::with_capacity(plaintext.len());
		for pair in plaintext.chars().zip(key.chars().cycle()) {
			ciphertext.push(char::from(CipherChar::from(pair.0) + CipherChar::from(pair.1)));
		}
		ciphertext
	}
	fn decipher(ciphertext: String, key: Self::Key) -> String
	{
		let mut plaintext = String::with_capacity(ciphertext.len());
		for pair in ciphertext.chars().zip(key.chars().cycle()) {
			plaintext.push(char::from(CipherChar::from(pair.0) - CipherChar::from(pair.1)));
		}
		plaintext
	}

	fn parse(key: &str) -> Option<Self::Key>
	{
		Some(String::from(key))
	}
}
