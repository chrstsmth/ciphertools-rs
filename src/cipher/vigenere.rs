use super::*;
use pallet::alph::*;
use std::iter::Iterator;

use std::convert::TryFrom;

pub struct Vigenere;

impl Vigenere {
	pub fn new() -> Vigenere {
		Vigenere
	}
}

impl Cipher for Vigenere {
	type Key = String;

	fn encipher(&self, plaintext: String, key: Self::Key) -> String
	{
		let mut ciphertext = String::with_capacity(plaintext.len());
		for pair in plaintext.chars().zip(key.chars().cycle()) {
			ciphertext.push(
				char::from(Alph::try_from(pair.0).unwrap() + Alph::try_from(pair.1).unwrap()));
		}
		ciphertext
	}
	fn decipher(&self, ciphertext: String, key: Self::Key) -> String
	{
		let mut plaintext = String::with_capacity(ciphertext.len());
		for pair in ciphertext.chars().zip(key.chars().cycle()) {
			plaintext.push(
				char::from(Alph::try_from(pair.0).unwrap() - Alph::try_from(pair.1).unwrap()));
		}
		plaintext
	}
}
