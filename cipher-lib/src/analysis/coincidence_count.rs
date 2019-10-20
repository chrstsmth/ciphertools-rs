extern crate colored;

use crate::pallet::alph::*;
use std::cmp;
use std::fmt;
use colored::*;

pub struct Coincidence {
	i: (usize, usize),
	len: usize,
}

pub struct CoincidenceFmt<'a> {
	coincidence: &'a Coincidence,
	text: &'a [Alph],
}

pub struct Coincidences {
	coincidences: Vec<Vec<Coincidence>>,
}

pub struct CoincidencesFmt<'a> {
	coincidences: &'a Coincidences,
	text: &'a [Alph],
}

impl Coincidences {
	pub fn new(text: &[Alph]) -> Self {
		let mut cs = Coincidences {
			coincidences: Vec::new(),
		};

		for n in 1..cmp::max(20,text.len()) {
			cs.coincidences.push(coincidence_count_at_offset(text, n));
		}
		cs
	}

	pub fn format<'a>(&'a self, text: &'a [Alph]) -> CoincidencesFmt<'a> {
		CoincidencesFmt {
			coincidences: self,
			text,
		}
	}
}

impl Coincidence {
	pub fn indices(&self) -> (usize, usize) {
		self.i
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn format<'a>(&'a self, text: &'a [Alph]) -> CoincidenceFmt<'a> {
		CoincidenceFmt {
			coincidence: self,
			text,
		}
	}
}
impl<'a> fmt::Display for CoincidencesFmt<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (i, cs) in self.coincidences.coincidences.iter().enumerate() {
			if !cs.is_empty() {
				writeln!(f, "offset: {}", i + 1)?;
			}
			for c in cs {
				write!(f, "{}", c.format(self.text))?;
			}
		}
		Ok(())
	}
}

impl<'a> fmt::Display for CoincidenceFmt<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (a, b) = (self.coincidence.i.0, self.coincidence.i.1);
		let len = self.coincidence.len;

		let r = a..a + len;
		let s = b..b + len;
		for (i, c) in self.text.iter().enumerate() {
			let in_r = r.contains(&i);
			let in_s = s.contains(&i);

			let style = if in_r && in_s {
				Some(Color::Magenta)
			} else if in_r {
				Some(Color::Red)
			} else if in_s {
				Some(Color::Blue)
			} else {
				None
			};

			match style {
				Some(col) => write!(f, "{}", c.to_string().color(col))?,
				None => write!(f, "{}", c)?,
			}
		}
		writeln!(f, "")
	}
}

fn coincidence_count_at_offset(text: &[Alph], n: usize) -> Vec<Coincidence> {
	let mut coincidences: Vec<Coincidence> = Vec::new();
	let mut push = |streak: Option<(usize, usize)>| {
		if let Some((i, j)) = streak {
			let len = j - i + 1;
			if len >= 2 {
				coincidences.push(Coincidence {
					i: (i, i + n),
					len,
				});
			}
		}
	};

	let mut streak: Option<(usize, usize)> = None;
	let iter_a = text.iter();
	let iter_b = iter_a.clone().skip(n);
	for (i, (a, b)) in iter_a.zip(iter_b).enumerate() {
		if *a == *b {
			streak = match streak {
				Some((s, _)) => Some((s, i)),
				None => Some((i, i)),
			};
		} else {
			push(streak);
			streak = None;
		}
	}
	push(streak);
	coincidences
}
