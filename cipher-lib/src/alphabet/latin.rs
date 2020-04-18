use std::convert::TryFrom;
use std::fmt;
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor, Unexpected};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use variant_count::*;
use enum_map::*;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, VariantCount, Enum)]
pub enum Latin {
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl Latin {
	pub const LENGTH: u32 = Self::VARIANT_COUNT as u32;

	pub fn iter() -> impl Iterator<Item = Self> {
		static ALPHS: [Latin; 26] = [
			Latin::A,
			Latin::B,
			Latin::C,
			Latin::D,
			Latin::E,
			Latin::F,
			Latin::G,
			Latin::H,
			Latin::I,
			Latin::J,
			Latin::K,
			Latin::L,
			Latin::M,
			Latin::N,
			Latin::O,
			Latin::P,
			Latin::Q,
			Latin::R,
			Latin::S,
			Latin::T,
			Latin::U,
			Latin::V,
			Latin::W,
			Latin::X,
			Latin::Y,
			Latin::Z];
		ALPHS.iter().map(|x| *x)
	}
}

impl IntoIterator for Latin {
	type Item = Latin;
	type IntoIter = impl Iterator<Item = Self>;

	fn into_iter(self) -> Self::IntoIter {
		Latin::iter().skip(u32::from(self) as usize)
	}
}

impl TryFrom<char> for Latin {
	type Error = &'static str;
	fn try_from(c: char) -> Result<Latin, Self::Error> {
		let c = c.to_ascii_lowercase();
		match c {
			'a' => Ok(Latin::A),
			'b' => Ok(Latin::B),
			'c' => Ok(Latin::C),
			'd' => Ok(Latin::D),
			'e' => Ok(Latin::E),
			'f' => Ok(Latin::F),
			'g' => Ok(Latin::G),
			'h' => Ok(Latin::H),
			'i' => Ok(Latin::I),
			'j' => Ok(Latin::J),
			'k' => Ok(Latin::K),
			'l' => Ok(Latin::L),
			'm' => Ok(Latin::M),
			'n' => Ok(Latin::N),
			'o' => Ok(Latin::O),
			'p' => Ok(Latin::P),
			'q' => Ok(Latin::Q),
			'r' => Ok(Latin::R),
			's' => Ok(Latin::S),
			't' => Ok(Latin::T),
			'u' => Ok(Latin::U),
			'v' => Ok(Latin::V),
			'w' => Ok(Latin::W),
			'x' => Ok(Latin::X),
			'y' => Ok(Latin::Y),
			'z' => Ok(Latin::Z),
			_ => return Err("Failed Latin::try_from::<char>")
		}
	}
}

impl From<Latin> for char {
	fn from(a: Latin) -> char {
		match a {
			Latin::A => 'a',
			Latin::B => 'b',
			Latin::C => 'c',
			Latin::D => 'd',
			Latin::E => 'e',
			Latin::F => 'f',
			Latin::G => 'g',
			Latin::H => 'h',
			Latin::I => 'i',
			Latin::J => 'j',
			Latin::K => 'k',
			Latin::L => 'l',
			Latin::M => 'm',
			Latin::N => 'n',
			Latin::O => 'o',
			Latin::P => 'p',
			Latin::Q => 'q',
			Latin::R => 'r',
			Latin::S => 's',
			Latin::T => 't',
			Latin::U => 'u',
			Latin::V => 'v',
			Latin::W => 'w',
			Latin::X => 'x',
			Latin::Y => 'y',
			Latin::Z => 'z',
		}
	}
}

impl fmt::Display for Latin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}

impl Distribution<Latin> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Latin {
		match rng.gen_range(0, Latin::VARIANT_COUNT - 1) {
			0 => Latin::A,
			1 => Latin::B,
			2 => Latin::C,
			3 => Latin::D,
			4 => Latin::E,
			5 => Latin::F,
			6 => Latin::G,
			7 => Latin::H,
			8 => Latin::I,
			9 => Latin::J,
			10 => Latin::K,
			11 => Latin::L,
			12 => Latin::M,
			13 => Latin::N,
			14 => Latin::O,
			15 => Latin::P,
			16 => Latin::Q,
			17 => Latin::R,
			18 => Latin::S,
			19 => Latin::T,
			20 => Latin::U,
			21 => Latin::V,
			22 => Latin::W,
			23 => Latin::X,
			24 => Latin::Y,
			_ => Latin::Z,
		}
	}
}

impl Serialize for Latin {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_char(char::from(*self))
		}
}

impl<'de> Deserialize<'de> for Latin {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		struct LatinVisitor;

		impl<'de> Visitor<'de> for LatinVisitor {
			type Value = Latin;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("alphabet character")
			}

			fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where
				E: de::Error,
			{
				let c = s.chars().next().unwrap();
				match Latin::try_from(c) {
					Ok(a) => Ok(a),
					Err(_) => Err(de::Error::invalid_value(Unexpected::Char(c), &self)),
				}
			}
		}

		deserializer.deserialize_char(LatinVisitor)
	}
}

impl From<Latin> for u32 {
	fn from(a: Latin) -> u32 {
		match a {
			Latin::A => 0,
			Latin::B => 1,
			Latin::C => 2,
			Latin::D => 3,
			Latin::E => 4,
			Latin::F => 5,
			Latin::G => 6,
			Latin::H => 7,
			Latin::I => 8,
			Latin::J => 9,
			Latin::K => 10,
			Latin::L => 11,
			Latin::M => 12,
			Latin::N => 13,
			Latin::O => 14,
			Latin::P => 15,
			Latin::Q => 16,
			Latin::R => 17,
			Latin::S => 18,
			Latin::T => 19,
			Latin::U => 20,
			Latin::V => 21,
			Latin::W => 22,
			Latin::X => 23,
			Latin::Y => 24,
			Latin::Z => 25,
		}
	}
}

impl TryFrom<u32> for Latin {
	type Error = &'static str;
	fn try_from(i: u32) -> Result<Latin, Self::Error> {
		match i {
			0 => Ok(Latin::A),
			1 => Ok(Latin::B),
			2 => Ok(Latin::C),
			3 => Ok(Latin::D),
			4 => Ok(Latin::E),
			5 => Ok(Latin::F),
			6 => Ok(Latin::G),
			7 => Ok(Latin::H),
			8 => Ok(Latin::I),
			9 => Ok(Latin::J),
			10 => Ok(Latin::K),
			11 => Ok(Latin::L),
			12 => Ok(Latin::M),
			13 => Ok(Latin::N),
			14 => Ok(Latin::O),
			15 => Ok(Latin::P),
			16 => Ok(Latin::Q),
			17 => Ok(Latin::R),
			18 => Ok(Latin::S),
			19 => Ok(Latin::T),
			20 => Ok(Latin::U),
			21 => Ok(Latin::V),
			22 => Ok(Latin::W),
			23 => Ok(Latin::X),
			24 => Ok(Latin::Y),
			25 => Ok(Latin::Z),
			_ => return Err("Failed u32::try_from::<Latin>")
		}
	}
}

impl Add for Latin {
	type Output = Latin;
	fn add(self, other: Latin) -> Latin {
		let a = u32::from(self);
		let b = u32::from(other);
		let c = (a + b) % Self::LENGTH;
		Latin::try_from(c).unwrap()
	}
}

impl Sub for Latin {
	type Output = Latin;
	fn sub(self, other: Latin) -> Latin {
		let a = u32::from(self);
		let b = u32::from(other);
		let c = (Self::LENGTH + a - b) % Self::LENGTH;
		Latin::try_from(c).unwrap()
	}
}
