use std::ops::Add;
use std::ops::Sub;
use pallet::alph::*;

use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
pub struct CipherChar(char);

impl Add for CipherChar {
	type Output = CipherChar ;
	fn add(self, other: CipherChar) -> CipherChar {
		let a = Alph::try_from(self.0);
		let b = Alph::try_from(other.0);
		if a.is_ok() && b.is_ok() {
			CipherChar::from(a.unwrap() + b.unwrap())
		} else {
			self
		}
	}
}

impl Sub for CipherChar {
	type Output = CipherChar ;
	fn sub(self, other: CipherChar) -> CipherChar {
		let a = Alph::try_from(self.0);
		let b = Alph::try_from(other.0);
		if a.is_ok() && b.is_ok() {
			CipherChar::from(a.unwrap() - b.unwrap())
		} else {
			self
		}
	}
}

impl From<Alph> for CipherChar {
	fn from(a: Alph) -> CipherChar {
		CipherChar(char::from(a))
	}
}

impl From<char> for CipherChar {
	fn from(c: char) -> CipherChar {
		CipherChar(c)
	}
}

impl From<CipherChar > for char {
	fn from(c: CipherChar) -> char {
		c.0
	}
}
