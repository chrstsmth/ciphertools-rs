pub mod vigenere;

pub trait Cipher {
	type Key;

	fn encipher(plaintext: String, k: Self::Key) -> String;
	fn decipher(ciphertext: String, k: Self::Key) -> String;
}


