use std::fmt;
use crate::key::*;
use crate::cipher::vigenere::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VigenereKey(pub String);

impl Key for VigenereKey {
	type Cipher = Vigenere;
}

impl From<String> for VigenereKey {
	fn from(key: String) -> VigenereKey
	{
		VigenereKey(key)
	}
}

impl fmt::Display for VigenereKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
