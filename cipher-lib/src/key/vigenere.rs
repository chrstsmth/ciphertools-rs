use crate::key::*;
use crate::cipher::vigenere::*;

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
