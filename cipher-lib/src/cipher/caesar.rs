use std::convert::TryFrom;
use crate::cipher::*;
use crate::cipher::vigenere::*;
use crate::key::caesar::*;
use crate::key::vigenere::*;
use crate::pallet::alph::*;
use crate::candidate::*;
use cipher_derive::*;
use min_max_heap::*;

#[derive(DictionaryAttack)]
pub struct Caesar;

impl Cipher for Caesar {
	type Key = CaesarKey;

	fn encipher(plaintext: &String, key: &Self::Key) -> String
	{
		Vigenere::encipher(plaintext, &VigenereKey::from(key.0.to_string()))
	}
	fn decipher(ciphertext: &String, key: &Self::Key) -> String
	{
		Vigenere::decipher(ciphertext, &VigenereKey::from(key.0.to_string()))
	}
}
