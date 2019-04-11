use super::*;
use std::iter::Iterator;

pub struct Vigenere;

impl Vigenere {
	pub fn new() -> Vigenere {
		Vigenere
	}
}

impl Cipher for Vigenere {
	type Key = Text;

	fn encipher(&self, plaintext: Text, key: Self::Key) -> Text
	{
		let mut ciphertext = Text{t: Vec::with_capacity(plaintext.t.len())};
		for pair in plaintext.t.iter().zip(key.t.iter().cycle()) {
			ciphertext.t.push(*pair.0 + *pair.1);
		}
		ciphertext
	}
	fn decipher(&self, ciphertext: Text, key: Self::Key) -> Text
	{
		let mut plaintext = Text{t: Vec::with_capacity(ciphertext.t.len())};
		for pair in ciphertext.t.iter().zip(key.t.iter().cycle()) {
			plaintext.t.push(*pair.0 - *pair.1);
		}
		plaintext
	}
}
