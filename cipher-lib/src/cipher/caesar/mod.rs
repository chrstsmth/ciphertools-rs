use crate::cipher::*;
use crate::cipher::vigenere::*;
use crate::key::caesar::*;
use crate::key::vigenere::*;
use crate::alphabet::latin::*;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Caesar;

impl Cipher for Caesar {
	const NAME: &'static str = "caesar";
	type Key = CaesarKey;
	type Config = ();

	fn encipher(plaintext: &str, key: &Self::Key, _config: &Self::Config) -> String
	{
		Vigenere::encipher(plaintext, &VigenereKey::from([Latin::from(key.clone())].as_ref()), &())
	}
	fn decipher(ciphertext: &str, key: &Self::Key, _config: &Self::Config) -> String
	{
		Vigenere::decipher(ciphertext, &VigenereKey::from([Latin::from(key.clone())].as_ref()), &())
	}
}
