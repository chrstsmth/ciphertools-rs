pub mod vigenere;
pub mod caesar;

use std::convert::TryFrom;
use crate::cipher::*;
use std::fmt;

pub trait Key: TryFrom<String> + fmt::Display {
	type Cipher: Cipher;
}
