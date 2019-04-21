use super::*;
use super::vigenere::*;
use std::convert::TryFrom;
use try_from_err::*;

pub struct Caesar;
pub struct CaesarKey(char);

impl Cipher for Caesar {
	type Key = CaesarKey;

	fn encipher(plaintext: String, key: Self::Key) -> String
	{
		Vigenere::encipher(plaintext, VigenereKey::from(key.0.to_string().as_str()))
	}
	fn decipher(ciphertext: String, key: Self::Key) -> String
	{
		Vigenere::decipher(ciphertext, VigenereKey::from(key.0.to_string().as_str()))
	}
}

impl TryFrom<&str> for CaesarKey {
	type Error = TryFromCharError;

	fn try_from(key: &str) -> Result<CaesarKey, TryFromCharError>
	{
		let mut chars = key.chars();

		match chars.next() {
			Some(first) => {
				match chars.next() {
					None => Ok(CaesarKey(first)),
					_ => Err(TryFromCharError),
				}
			}
			_ => Err(TryFromCharError),
		}
	}
}

impl From<char> for CaesarKey{
	fn from(key: char) -> CaesarKey
	{
		CaesarKey(key)
	}
}
