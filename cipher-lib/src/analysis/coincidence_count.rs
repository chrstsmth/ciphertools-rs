use std::iter;
use crate::pallet::alph::*;
use std::cmp;

pub struct CoincidencesTable<'a> {
	coincidences: Vec<CoincidencesData>,
	text: &'a [Alph],
}

pub struct Coincidences<'a> {
	coincidences: &'a CoincidencesData,
	text: &'a [Alph],
}

pub struct Coincidence<'a> {
	coincidence: &'a CoincidenceData,
	text: &'a [Alph],
}

pub struct CoincidencesData {
	coincidences: Vec<CoincidenceData>,
}

pub struct CoincidenceData {
	i: (usize, usize),
	len: usize,
}

pub struct CoincidencesTableIter<'a> {
	it: <&'a Vec<CoincidencesData> as IntoIterator>::IntoIter,
	text: &'a[Alph],
}

pub struct CoincidencesIter<'a> {
	it: <&'a Vec<CoincidenceData> as IntoIterator>::IntoIter,
	text: &'a[Alph],
}

impl<'a> iter::IntoIterator for &'a CoincidencesTable<'a> {
	type Item = Coincidences<'a>;
	type IntoIter = CoincidencesTableIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		CoincidencesTableIter {
			it: self.coincidences.iter(),
			text: self.text,
		}
	}
}

impl<'a> iter::Iterator for CoincidencesTableIter<'a> {
	type Item = Coincidences<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let coincidences = self.it.next();
		match coincidences {
			Some(coincidences) => {
				Some(Coincidences {
					coincidences,
					text: self.text,
				})
			},
			None => None,
		}
	}
}

impl<'a> iter::IntoIterator for &'a Coincidences<'a> {
	type Item = Coincidence<'a>;
	type IntoIter = CoincidencesIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		CoincidencesIter {
			it: self.coincidences.coincidences.iter(),
			text: self.text,
		}
	}
}

impl<'a> iter::Iterator for CoincidencesIter<'a> {
	type Item = Coincidence<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let coincidence = self.it.next();
		match coincidence {
			Some(coincidence) => {
				Some(Coincidence {
					coincidence,
					text: self.text,
				})
			},
			None => None,
		}
	}
}

impl<'a> CoincidencesTable<'a> {
	pub fn new(text: &'a [Alph]) -> Self {
		Self::with_length(32, text)
	}

	pub fn with_length(n: usize, text: &'a [Alph]) -> Self {
		let mut table = CoincidencesTable {
			coincidences: Vec::new(),
			text: text,
		};

		for n in 1..cmp::min(n, text.len() / 2) {
			table.coincidences.push(CoincidencesData::with_offset(text, n));
		}
		table
	}

	pub fn len(&self) -> usize {
		self.coincidences.len()
	}

	pub fn at(&'a self, i: usize) -> Coincidences<'a> {
		Coincidences {
			coincidences: &self.coincidences[i],
			text: self.text,
		}
	}

	pub fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
		self.into_iter()
	}
}

impl CoincidencesData {
	pub fn with_offset(text: &[Alph], n: usize) -> Self {
		let mut c = CoincidencesData {
			coincidences: Vec::new(),
		};
		let mut push = |streak: Option<(usize, usize)>| {
			if let Some((i, j)) = streak {
				let len = j - i + 1;
				if len >= 2 {
					c.coincidences.push(CoincidenceData {
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

		c.coincidences.sort_by(|a, b| a.len.cmp(&b.len));
		c
	}
}

impl<'a> Coincidences<'a> {
	pub fn len(&self) {
		self.coincidences.coincidences.len();
	}

	pub fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
		self.into_iter()
	}
}

impl<'a> Coincidence<'a> {
	pub fn len(&self) -> usize {
		self.coincidence.len
	}
	pub fn indices(&self) -> (usize, usize) {
		self.coincidence.i
	}
}
