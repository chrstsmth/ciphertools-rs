use std::fmt;
use std::convert::TryFrom;
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor, Unexpected};
use super::try_from_err::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Lang {
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Space
}

impl From<Lang> for char {
	fn from(a: Lang) -> char {
		match a {
			Lang::A => 'a',
			Lang::B => 'b',
			Lang::C => 'c',
			Lang::D => 'd',
			Lang::E => 'e',
			Lang::F => 'f',
			Lang::G => 'g',
			Lang::H => 'h',
			Lang::I => 'i',
			Lang::J => 'j',
			Lang::K => 'k',
			Lang::L => 'l',
			Lang::M => 'm',
			Lang::N => 'n',
			Lang::O => 'o',
			Lang::P => 'p',
			Lang::Q => 'q',
			Lang::R => 'r',
			Lang::S => 's',
			Lang::T => 't',
			Lang::U => 'u',
			Lang::V => 'v',
			Lang::W => 'w',
			Lang::X => 'x',
			Lang::Y => 'y',
			Lang::Z => 'z',
			Lang::Space => ' ',
		}
	}
}

impl TryFrom<char> for Lang {
	type Error = TryFromCharError;
	fn try_from(c: char) -> Result<Lang, TryFromCharError> {
		match c {
			'a' => Ok(Lang::A),
			'b' => Ok(Lang::B),
			'c' => Ok(Lang::C),
			'd' => Ok(Lang::D),
			'e' => Ok(Lang::E),
			'f' => Ok(Lang::F),
			'g' => Ok(Lang::G),
			'h' => Ok(Lang::H),
			'i' => Ok(Lang::I),
			'j' => Ok(Lang::J),
			'k' => Ok(Lang::K),
			'l' => Ok(Lang::L),
			'm' => Ok(Lang::M),
			'n' => Ok(Lang::N),
			'o' => Ok(Lang::O),
			'p' => Ok(Lang::P),
			'q' => Ok(Lang::Q),
			'r' => Ok(Lang::R),
			's' => Ok(Lang::S),
			't' => Ok(Lang::T),
			'u' => Ok(Lang::U),
			'v' => Ok(Lang::V),
			'w' => Ok(Lang::W),
			'x' => Ok(Lang::X),
			'y' => Ok(Lang::Y),
			'z' => Ok(Lang::Z),
			'A' => Ok(Lang::A),
			'B' => Ok(Lang::B),
			'C' => Ok(Lang::C),
			'D' => Ok(Lang::D),
			'E' => Ok(Lang::E),
			'F' => Ok(Lang::F),
			'G' => Ok(Lang::G),
			'H' => Ok(Lang::H),
			'I' => Ok(Lang::I),
			'J' => Ok(Lang::J),
			'K' => Ok(Lang::K),
			'L' => Ok(Lang::L),
			'M' => Ok(Lang::M),
			'N' => Ok(Lang::N),
			'O' => Ok(Lang::O),
			'P' => Ok(Lang::P),
			'Q' => Ok(Lang::Q),
			'R' => Ok(Lang::R),
			'S' => Ok(Lang::S),
			'T' => Ok(Lang::T),
			'U' => Ok(Lang::U),
			'V' => Ok(Lang::V),
			'W' => Ok(Lang::W),
			'X' => Ok(Lang::X),
			'Y' => Ok(Lang::Y),
			'Z' => Ok(Lang::Z),
			' ' => Ok(Lang::Space),
			_ => Err(TryFromCharError),
		}
	}
}

impl Serialize for Lang {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_char(char::from(*self))
		}
}

impl<'de> Deserialize<'de> for Lang {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		struct LangVisitor;

		impl<'de> Visitor<'de> for LangVisitor {
			type Value = Lang;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("language min character")
			}

			fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where
				E: de::Error,
			{
				let c = s.chars().next().unwrap();
				match Lang::try_from(c) {
					Ok(a) => Ok(a),
					_ => Err(de::Error::invalid_value(Unexpected::Char(c), &self)),
				}
			}
		}

		deserializer.deserialize_char(LangVisitor)
	}
}

impl fmt::Display for Lang  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}
