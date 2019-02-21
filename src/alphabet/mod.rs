use std::fmt;
use std::convert::TryFrom;
use std::error;

#[derive(Copy, Clone)]
pub enum Alphabet {
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

#[derive(Debug,Clone)]
pub struct TryFromCharError;

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

impl error::Error for TryFromCharError {
	fn description(&self) -> &str {
		"No conversion available"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for TryFromCharError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TODO")
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
			_ => Err(TryFromCharError),
		}
	}
}

impl fmt::Display for Alphabet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}

