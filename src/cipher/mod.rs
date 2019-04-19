pub mod vigenere;

pub trait Cipher {
	type Key;

	fn encipher(&self, plaintext: String, k: Self::Key) -> String;
	fn decipher(&self, ciphertext: String, k: Self::Key) -> String;
}


