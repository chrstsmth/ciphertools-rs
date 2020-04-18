use crate::alphabet::latin::*;
use std::convert::TryFrom;
use std::ops::{Add, Sub};

pub struct CaesarChar(char);

impl From<char> for CaesarChar {
	fn from(c: char) -> CaesarChar {
		CaesarChar(c)
	}
}

impl From<CaesarChar> for char {
	fn from(c: CaesarChar) -> char {
		c.0
	}
}

impl Add<Latin> for CaesarChar {
	type Output = Self;
	fn add(self, other: Latin) -> CaesarChar {
		match Latin::try_from(self.0) {
			Ok(b) => Self::from(char::from(other + b)),
			Err(_) => self,
		}
	}
}

impl Sub<Latin> for CaesarChar {
	type Output = Self;
	fn sub(self, other: Latin) -> CaesarChar {
		match Latin::try_from(self.0) {
			Ok(b) => Self::from(char::from(other - b)),
			Err(_) => self,
		}
	}
}

impl Add for CaesarChar {
	type Output = CaesarChar;
	fn add(self, other: CaesarChar) -> CaesarChar {
		match Latin::try_from(other.0) {
			Ok(b) => self + b,
			Err(_) => self,
		}
	}
}

impl Sub for CaesarChar {
	type Output = CaesarChar;
	fn sub(self, other: CaesarChar) -> CaesarChar {
		match Latin::try_from(other.0) {
			Ok(b) => self - b,
			Err(_) => self,
		}
	}
}
