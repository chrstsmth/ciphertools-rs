use std::fmt;
use std::convert::TryFrom;
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor, Unexpected};
use super::try_from_err::*;

#[derive(Copy, Clone, PartialEq)]
pub enum LangMin {
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Space
}

impl From<LangMin> for char {
	fn from(a: LangMin) -> char {
		match a {
			LangMin::A => 'a',
			LangMin::B => 'b',
			LangMin::C => 'c',
			LangMin::D => 'd',
			LangMin::E => 'e',
			LangMin::F => 'f',
			LangMin::G => 'g',
			LangMin::H => 'h',
			LangMin::I => 'i',
			LangMin::J => 'j',
			LangMin::K => 'k',
			LangMin::L => 'l',
			LangMin::M => 'm',
			LangMin::N => 'n',
			LangMin::O => 'o',
			LangMin::P => 'p',
			LangMin::Q => 'q',
			LangMin::R => 'r',
			LangMin::S => 's',
			LangMin::T => 't',
			LangMin::U => 'u',
			LangMin::V => 'v',
			LangMin::W => 'w',
			LangMin::X => 'x',
			LangMin::Y => 'y',
			LangMin::Z => 'z',
			LangMin::Space => ' ',
		}
	}
}

impl TryFrom<char> for LangMin {
	type Error = TryFromCharError;
	fn try_from(c: char) -> Result<LangMin, TryFromCharError> {
		match c {
			'a' => Ok(LangMin::A),
			'b' => Ok(LangMin::B),
			'c' => Ok(LangMin::C),
			'd' => Ok(LangMin::D),
			'e' => Ok(LangMin::E),
			'f' => Ok(LangMin::F),
			'g' => Ok(LangMin::G),
			'h' => Ok(LangMin::H),
			'i' => Ok(LangMin::I),
			'j' => Ok(LangMin::J),
			'k' => Ok(LangMin::K),
			'l' => Ok(LangMin::L),
			'm' => Ok(LangMin::M),
			'n' => Ok(LangMin::N),
			'o' => Ok(LangMin::O),
			'p' => Ok(LangMin::P),
			'q' => Ok(LangMin::Q),
			'r' => Ok(LangMin::R),
			's' => Ok(LangMin::S),
			't' => Ok(LangMin::T),
			'u' => Ok(LangMin::U),
			'v' => Ok(LangMin::V),
			'w' => Ok(LangMin::W),
			'x' => Ok(LangMin::X),
			'y' => Ok(LangMin::Y),
			'z' => Ok(LangMin::Z),
			'A' => Ok(LangMin::A),
			'B' => Ok(LangMin::B),
			'C' => Ok(LangMin::C),
			'D' => Ok(LangMin::D),
			'E' => Ok(LangMin::E),
			'F' => Ok(LangMin::F),
			'G' => Ok(LangMin::G),
			'H' => Ok(LangMin::H),
			'I' => Ok(LangMin::I),
			'J' => Ok(LangMin::J),
			'K' => Ok(LangMin::K),
			'L' => Ok(LangMin::L),
			'M' => Ok(LangMin::M),
			'N' => Ok(LangMin::N),
			'O' => Ok(LangMin::O),
			'P' => Ok(LangMin::P),
			'Q' => Ok(LangMin::Q),
			'R' => Ok(LangMin::R),
			'S' => Ok(LangMin::S),
			'T' => Ok(LangMin::T),
			'U' => Ok(LangMin::U),
			'V' => Ok(LangMin::V),
			'W' => Ok(LangMin::W),
			'X' => Ok(LangMin::X),
			'Y' => Ok(LangMin::Y),
			'Z' => Ok(LangMin::Z),
			' ' => Ok(LangMin::Space),
			_ => Err(TryFromCharError),
		}
	}
}

impl Serialize for LangMin {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_char(char::from(*self))
		}
}

impl<'de> Deserialize<'de> for LangMin {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		struct AlphabetVisitor;

		impl<'de> Visitor<'de> for AlphabetVisitor {
			type Value = LangMin;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("language min character")
			}

			fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where
				E: de::Error,
			{
				let c = s.chars().next().unwrap();
				match LangMin::try_from(c) {
					Ok(a) => Ok(a),
					_ => Err(de::Error::invalid_value(Unexpected::Char(c), &self)),
				}
			}
		}

		deserializer.deserialize_char(AlphabetVisitor)
	}
}

impl fmt::Display for LangMin  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}
