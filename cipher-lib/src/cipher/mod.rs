pub mod vigenere;
pub mod caesar;

use crate::key::*;

pub trait Cipher {
	type Key: Key;

	fn encipher(plaintext: String, k: Self::Key) -> String;
	fn decipher(ciphertext: String, k: Self::Key) -> String;
}

