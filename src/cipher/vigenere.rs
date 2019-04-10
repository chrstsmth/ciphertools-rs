use super::*;
use std::iter::Iterator;

pub struct Vigenere;

impl Vigenere {
	pub fn new() -> Vigenere {
		Vigenere
	}
}

impl Cipher for Vigenere {
	type Key = Vec<Alphabet>;

	fn encipher(&self, plaintext: &[Alphabet], key: Self::Key) -> Vec<Alphabet>
	{
		let mut ciphertext = Vec::with_capacity(plaintext.len());
		for pair in plaintext.iter().zip(key.iter().cycle()) {
			ciphertext.push(*pair.0 + *pair.1);
		}
		ciphertext
	}
	fn decipher(&self, ciphertext: &[Alphabet], key: Self::Key) -> Vec<Alphabet>
	{
		let mut plaintext = Vec::with_capacity(ciphertext.len());
		for pair in ciphertext.iter().zip(key.iter().cycle()) {
			plaintext.push(*pair.0 - *pair.1);
		}
		plaintext
	}
}
