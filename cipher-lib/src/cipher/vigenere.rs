use crate::cipher::*;
use crate::key::vigenere::*;
use crate::pallet::cipher_char::*;
use std::iter::Iterator;

pub struct Vigenere;

impl Cipher for Vigenere {
	type Key = VigenereKey;

	fn encipher(plaintext: String, key: Self::Key) -> String
	{
		let mut ciphertext = String::with_capacity(plaintext.len());
		for pair in plaintext.chars().zip(key.0.chars().cycle()) {
			ciphertext.push(char::from(CipherChar::from(pair.0) + CipherChar::from(pair.1)));
		}
		ciphertext
	}
	fn decipher(ciphertext: String, key: Self::Key) -> String
	{
		let mut plaintext = String::with_capacity(ciphertext.len());
		for pair in ciphertext.chars().zip(key.0.chars().cycle()) {
			plaintext.push(char::from(CipherChar::from(pair.0) - CipherChar::from(pair.1)));
		}
		plaintext
	}
}
