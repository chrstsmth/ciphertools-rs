use std::fmt;
use std::convert::TryFrom;
use std::error;
use serde::ser::{Serialize, Serializer};

#[derive(Copy, Clone)]
pub enum Alphabet {
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

#[derive(Debug,Clone)]
pub struct TryFromCharError;

#[derive(Debug,Clone)]
pub struct TryFromIntError;

impl From<Alphabet> for char {
	fn from(a: Alphabet) -> char {
		match a {
			Alphabet::A => 'a',
			Alphabet::B => 'b',
			Alphabet::C => 'c',
			Alphabet::D => 'd',
			Alphabet::E => 'e',
			Alphabet::F => 'f',
			Alphabet::G => 'g',
			Alphabet::H => 'h',
			Alphabet::I => 'i',
			Alphabet::J => 'j',
			Alphabet::K => 'k',
			Alphabet::L => 'l',
			Alphabet::M => 'm',
			Alphabet::N => 'n',
			Alphabet::O => 'o',
			Alphabet::P => 'p',
			Alphabet::Q => 'q',
			Alphabet::R => 'r',
			Alphabet::S => 's',
			Alphabet::T => 't',
			Alphabet::U => 'u',
			Alphabet::V => 'v',
			Alphabet::W => 'w',
			Alphabet::X => 'x',
			Alphabet::Y => 'y',
			Alphabet::Z => 'z',
		}
	}
}

impl From<Alphabet> for usize {
	fn from(a: Alphabet) -> usize {
		match a {
			Alphabet::A => 0,
			Alphabet::B => 1,
			Alphabet::C => 2,
			Alphabet::D => 3,
			Alphabet::E => 4,
			Alphabet::F => 5,
			Alphabet::G => 6,
			Alphabet::H => 7,
			Alphabet::I => 8,
			Alphabet::J => 9,
			Alphabet::K => 10,
			Alphabet::L => 11,
			Alphabet::M => 12,
			Alphabet::N => 13,
			Alphabet::O => 14,
			Alphabet::P => 15,
			Alphabet::Q => 16,
			Alphabet::R => 17,
			Alphabet::S => 18,
			Alphabet::T => 19,
			Alphabet::U => 20,
			Alphabet::V => 21,
			Alphabet::W => 22,
			Alphabet::X => 23,
			Alphabet::Y => 24,
			Alphabet::Z => 25,
		}
	}
}

impl TryFrom<char> for Alphabet {
	type Error = TryFromCharError;
	fn try_from(c: char) -> Result<Alphabet, TryFromCharError> {
		match c {
			'a' => Ok(Alphabet::A),
			'b' => Ok(Alphabet::B),
			'c' => Ok(Alphabet::C),
			'd' => Ok(Alphabet::D),
			'e' => Ok(Alphabet::E),
			'f' => Ok(Alphabet::F),
			'g' => Ok(Alphabet::G),
			'h' => Ok(Alphabet::H),
			'i' => Ok(Alphabet::I),
			'j' => Ok(Alphabet::J),
			'k' => Ok(Alphabet::K),
			'l' => Ok(Alphabet::L),
			'm' => Ok(Alphabet::M),
			'n' => Ok(Alphabet::N),
			'o' => Ok(Alphabet::O),
			'p' => Ok(Alphabet::P),
			'q' => Ok(Alphabet::Q),
			'r' => Ok(Alphabet::R),
			's' => Ok(Alphabet::S),
			't' => Ok(Alphabet::T),
			'u' => Ok(Alphabet::U),
			'v' => Ok(Alphabet::V),
			'w' => Ok(Alphabet::W),
			'x' => Ok(Alphabet::X),
			'y' => Ok(Alphabet::Y),
			'z' => Ok(Alphabet::Z),
			'A' => Ok(Alphabet::A),
			'B' => Ok(Alphabet::B),
			'C' => Ok(Alphabet::C),
			'D' => Ok(Alphabet::D),
			'E' => Ok(Alphabet::E),
			'F' => Ok(Alphabet::F),
			'G' => Ok(Alphabet::G),
			'H' => Ok(Alphabet::H),
			'I' => Ok(Alphabet::I),
			'J' => Ok(Alphabet::J),
			'K' => Ok(Alphabet::K),
			'L' => Ok(Alphabet::L),
			'M' => Ok(Alphabet::M),
			'N' => Ok(Alphabet::N),
			'O' => Ok(Alphabet::O),
			'P' => Ok(Alphabet::P),
			'Q' => Ok(Alphabet::Q),
			'R' => Ok(Alphabet::R),
			'S' => Ok(Alphabet::S),
			'T' => Ok(Alphabet::T),
			'U' => Ok(Alphabet::U),
			'V' => Ok(Alphabet::V),
			'W' => Ok(Alphabet::W),
			'X' => Ok(Alphabet::X),
			'Y' => Ok(Alphabet::Y),
			'Z' => Ok(Alphabet::Z),
			_ => Err(TryFromCharError),
		}
	}
}

impl TryFrom<usize> for Alphabet {
	type Error = TryFromIntError;
	fn try_from(i: usize) -> Result<Alphabet, TryFromIntError> {
		match i {
			0 => Ok(Alphabet::A),
			1 => Ok(Alphabet::B),
			2 => Ok(Alphabet::C),
			3 => Ok(Alphabet::D),
			4 => Ok(Alphabet::E),
			5 => Ok(Alphabet::F),
			6 => Ok(Alphabet::G),
			7 => Ok(Alphabet::H),
			8 => Ok(Alphabet::I),
			9 => Ok(Alphabet::J),
			10 => Ok(Alphabet::K),
			11 => Ok(Alphabet::L),
			12 => Ok(Alphabet::M),
			13 => Ok(Alphabet::N),
			14 => Ok(Alphabet::O),
			15 => Ok(Alphabet::P),
			16 => Ok(Alphabet::Q),
			17 => Ok(Alphabet::R),
			18 => Ok(Alphabet::S),
			19 => Ok(Alphabet::T),
			20 => Ok(Alphabet::U),
			21 => Ok(Alphabet::V),
			22 => Ok(Alphabet::W),
			23 => Ok(Alphabet::X),
			24 => Ok(Alphabet::Y),
			25 => Ok(Alphabet::Z),
			_ => Err(TryFromIntError),
		}
	}
}

impl Serialize for Alphabet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_char(char::from(*self))
		}
}

impl fmt::Display for Alphabet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}

impl error::Error for TryFromCharError {
	fn description(&self) -> &str {
		"no conversion available"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for TryFromCharError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "no conversion available")
	}
}

impl error::Error for TryFromIntError {
	fn description(&self) -> &str {
		"no conversion available"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for TryFromIntError  {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "no conversion available")
	}
}
