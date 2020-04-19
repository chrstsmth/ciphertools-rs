pub mod vigenere;
pub mod caesar;

use crate::key::*;

pub trait Cipher: Clone + Eq + Ord {
	const NAME: &'static str;
	type Key: Key;
	type Config;

	fn encipher(plaintext: &str, k: &Self::Key, config: &Self::Config) -> String;
	fn decipher(ciphertext: &str, k: &Self::Key, config: &Self::Config) -> String;
}
