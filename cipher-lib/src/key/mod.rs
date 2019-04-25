pub mod vigenere;
pub mod caesar;

use std::convert::TryFrom;
use crate::cipher::*;

pub trait Key: TryFrom<String> {
	type Cipher: Cipher;
}
