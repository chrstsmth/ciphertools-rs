use crate::cipher::*;
use crate::key::vigenere::*;
use crate::pallet::cipher_char::*;
use std::iter::Iterator;
use cipher_derive::*;

#[cfg(test)]
mod tests;

#[derive(DictionaryAttack, HillClimb, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vigenere;

impl Cipher for Vigenere {
	const NAME: &'static str = "vigenere";
	type Key = VigenereKey;

	fn encipher(plaintext: &str, key: &Self::Key) -> String
	{
		let mut ciphertext = String::with_capacity(plaintext.len());
		for (p, k) in plaintext.chars().zip(key.0.iter().cycle()) {
			ciphertext.push(char::from(CipherChar::from(p) + CipherChar::from(*k)));
		}
		ciphertext
	}
	fn decipher(ciphertext: &str, key: &Self::Key) -> String
	{
		let mut plaintext = String::with_capacity(ciphertext.len());
		for (p, k) in ciphertext.chars().zip(key.0.iter().cycle()) {
			plaintext.push(char::from(CipherChar::from(p) - CipherChar::from(*k)));
		}
		plaintext
	}
}

