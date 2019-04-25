pub mod vigenere;
pub mod caesar;

pub trait Key {
	type Cipher;
}
