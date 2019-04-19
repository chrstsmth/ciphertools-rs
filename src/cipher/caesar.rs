use super::*;
use super::vigenere::*;

pub struct Caesar;

impl Cipher for Caesar {
	type Key = char;

	fn encipher(plaintext: String, key: Self::Key) -> String
	{
		Vigenere::encipher(plaintext, key.to_string())
	}
	fn decipher(ciphertext: String, key: Self::Key) -> String
	{
		Vigenere::decipher(ciphertext, key.to_string())
	}

	fn parse(key: &str) -> Option<Self::Key>
	{
		let mut chars = key.chars();

		match chars.next() {
			Some(first) => {
				match chars.next() {
					None => Some(first),
					_ => None,
				}
			}
			_ => None,
		}
	}
}
