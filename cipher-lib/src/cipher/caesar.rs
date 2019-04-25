use super::*;
use crate::cipher::vigenere::*;
use crate::key::caesar::*;
use crate::key::vigenere::*;

pub struct Caesar;

impl Cipher for Caesar {
	type Key = CaesarKey;

	fn encipher(plaintext: String, key: Self::Key) -> String
	{
		Vigenere::encipher(plaintext, VigenereKey::from(key.0.to_string()))
	}
	fn decipher(ciphertext: String, key: Self::Key) -> String
	{
		Vigenere::decipher(ciphertext, VigenereKey::from(key.0.to_string()))
	}
}
