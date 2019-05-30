use std::fmt;
use std::convert::TryFrom;
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor, Unexpected};
use std::ops::{Add, Sub};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::try_from_err::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Alph {
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl Alph {
	pub const SIZE: usize = 26;
}

impl From<Alph> for char {
	fn from(a: Alph) -> char {
		match a {
			Alph::A => 'a',
			Alph::B => 'b',
			Alph::C => 'c',
			Alph::D => 'd',
			Alph::E => 'e',
			Alph::F => 'f',
			Alph::G => 'g',
			Alph::H => 'h',
			Alph::I => 'i',
			Alph::J => 'j',
			Alph::K => 'k',
			Alph::L => 'l',
			Alph::M => 'm',
			Alph::N => 'n',
			Alph::O => 'o',
			Alph::P => 'p',
			Alph::Q => 'q',
			Alph::R => 'r',
			Alph::S => 's',
			Alph::T => 't',
			Alph::U => 'u',
			Alph::V => 'v',
			Alph::W => 'w',
			Alph::X => 'x',
			Alph::Y => 'y',
			Alph::Z => 'z',
		}
	}
}

impl TryFrom<char> for Alph {
	type Error = TryFromCharError;
	fn try_from(c: char) -> Result<Alph, TryFromCharError> {
		match c {
			'a' => Ok(Alph::A),
			'b' => Ok(Alph::B),
			'c' => Ok(Alph::C),
			'd' => Ok(Alph::D),
			'e' => Ok(Alph::E),
			'f' => Ok(Alph::F),
			'g' => Ok(Alph::G),
			'h' => Ok(Alph::H),
			'i' => Ok(Alph::I),
			'j' => Ok(Alph::J),
			'k' => Ok(Alph::K),
			'l' => Ok(Alph::L),
			'm' => Ok(Alph::M),
			'n' => Ok(Alph::N),
			'o' => Ok(Alph::O),
			'p' => Ok(Alph::P),
			'q' => Ok(Alph::Q),
			'r' => Ok(Alph::R),
			's' => Ok(Alph::S),
			't' => Ok(Alph::T),
			'u' => Ok(Alph::U),
			'v' => Ok(Alph::V),
			'w' => Ok(Alph::W),
			'x' => Ok(Alph::X),
			'y' => Ok(Alph::Y),
			'z' => Ok(Alph::Z),
			'A' => Ok(Alph::A),
			'B' => Ok(Alph::B),
			'C' => Ok(Alph::C),
			'D' => Ok(Alph::D),
			'E' => Ok(Alph::E),
			'F' => Ok(Alph::F),
			'G' => Ok(Alph::G),
			'H' => Ok(Alph::H),
			'I' => Ok(Alph::I),
			'J' => Ok(Alph::J),
			'K' => Ok(Alph::K),
			'L' => Ok(Alph::L),
			'M' => Ok(Alph::M),
			'N' => Ok(Alph::N),
			'O' => Ok(Alph::O),
			'P' => Ok(Alph::P),
			'Q' => Ok(Alph::Q),
			'R' => Ok(Alph::R),
			'S' => Ok(Alph::S),
			'T' => Ok(Alph::T),
			'U' => Ok(Alph::U),
			'V' => Ok(Alph::V),
			'W' => Ok(Alph::W),
			'X' => Ok(Alph::X),
			'Y' => Ok(Alph::Y),
			'Z' => Ok(Alph::Z),
			_ => Err(TryFromCharError),
		}
	}
}

impl From<Alph> for usize {
	fn from(a: Alph) -> usize {
		match a {
			Alph::A => 0,
			Alph::B => 1,
			Alph::C => 2,
			Alph::D => 3,
			Alph::E => 4,
			Alph::F => 5,
			Alph::G => 6,
			Alph::H => 7,
			Alph::I => 8,
			Alph::J => 9,
			Alph::K => 10,
			Alph::L => 11,
			Alph::M => 12,
			Alph::N => 13,
			Alph::O => 14,
			Alph::P => 15,
			Alph::Q => 16,
			Alph::R => 17,
			Alph::S => 18,
			Alph::T => 19,
			Alph::U => 20,
			Alph::V => 21,
			Alph::W => 22,
			Alph::X => 23,
			Alph::Y => 24,
			Alph::Z => 25,
		}
	}
}

impl TryFrom<usize> for Alph {
	type Error = TryFromIntError;
	fn try_from(i: usize) -> Result<Alph, TryFromIntError> {
		match i {
			0 => Ok(Alph::A),
			1 => Ok(Alph::B),
			2 => Ok(Alph::C),
			3 => Ok(Alph::D),
			4 => Ok(Alph::E),
			5 => Ok(Alph::F),
			6 => Ok(Alph::G),
			7 => Ok(Alph::H),
			8 => Ok(Alph::I),
			9 => Ok(Alph::J),
			10 => Ok(Alph::K),
			11 => Ok(Alph::L),
			12 => Ok(Alph::M),
			13 => Ok(Alph::N),
			14 => Ok(Alph::O),
			15 => Ok(Alph::P),
			16 => Ok(Alph::Q),
			17 => Ok(Alph::R),
			18 => Ok(Alph::S),
			19 => Ok(Alph::T),
			20 => Ok(Alph::U),
			21 => Ok(Alph::V),
			22 => Ok(Alph::W),
			23 => Ok(Alph::X),
			24 => Ok(Alph::Y),
			25 => Ok(Alph::Z),
			_ => Err(TryFromIntError),
		}
	}
}

impl Distribution<Alph> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Alph {
		match rng.gen_range(0, Alph::SIZE - 1) {
			0 => Alph::A,
			1 => Alph::B,
			2 => Alph::C,
			3 => Alph::D,
			4 => Alph::E,
			5 => Alph::F,
			6 => Alph::G,
			7 => Alph::H,
			8 => Alph::I,
			9 => Alph::J,
			10 => Alph::K,
			11 => Alph::L,
			12 => Alph::M,
			13 => Alph::N,
			14 => Alph::O,
			15 => Alph::P,
			16 => Alph::Q,
			17 => Alph::R,
			18 => Alph::S,
			19 => Alph::T,
			20 => Alph::U,
			21 => Alph::V,
			22 => Alph::W,
			23 => Alph::X,
			24 => Alph::Y,
			_ => Alph::Z,
		}
	}
}

impl Add for Alph {
	type Output = Alph;
	fn add(self, other: Alph) -> Alph {
		let a = (usize::from(self) + usize::from(other)) % Self::SIZE;
		Alph::try_from(a).unwrap()
	}
}

impl Sub for Alph {
	type Output = Alph;
	fn sub(self, other: Alph) -> Alph {
		let a = (Self::SIZE + usize::from(self) - usize::from(other)) % Self::SIZE;
		Alph::try_from(a).unwrap()
	}
}

impl Serialize for Alph {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_char(char::from(*self))
		}
}

impl<'de> Deserialize<'de> for Alph {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		struct AlphVisitor;

		impl<'de> Visitor<'de> for AlphVisitor {
			type Value = Alph;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("alphabet character")
			}

			fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where
				E: de::Error,
			{
				let c = s.chars().next().unwrap();
				match Alph::try_from(c) {
					Ok(a) => Ok(a),
					_ => Err(de::Error::invalid_value(Unexpected::Char(c), &self)),
				}
			}
		}

		deserializer.deserialize_char(AlphVisitor)
	}
}

impl fmt::Display for Alph {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}
