use crate::cipher::*;
use crate::key::vigenere::*;
use crate::pallet::cipher_char::*;
use std::iter::Iterator;
use cipher_derive::*;

use std::convert::TryFrom;
use crate::pallet::alph::*;
use min_max_heap::*;

#[derive(DictionaryAttack)]
pub struct Vigenere;

impl Cipher for Vigenere {
	type Key = VigenereKey;

	fn encipher(plaintext: &String, key: &Self::Key) -> String
	{
		let mut ciphertext = String::with_capacity(plaintext.len());
		for pair in plaintext.chars().zip(key.0.chars().cycle()) {
			ciphertext.push(char::from(CipherChar::from(pair.0) + CipherChar::from(pair.1)));
		}
		ciphertext
	}
	fn decipher(ciphertext: &String, key: &Self::Key) -> String
	{
		let mut plaintext = String::with_capacity(ciphertext.len());
		for pair in ciphertext.chars().zip(key.0.chars().cycle()) {
			plaintext.push(char::from(CipherChar::from(pair.0) - CipherChar::from(pair.1)));
		}
		plaintext
	}
}
