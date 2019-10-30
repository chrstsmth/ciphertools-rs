use std::iter;
use crate::pallet::alph::*;
use std::cmp;

pub struct CoincidencesTable<'a> {
	coincidences: Vec<CoincidencesData>,
	m_text: &'a [Alph],
}

pub struct CoincidencesData {
	coincidences: Vec<CoincidenceData>,
}

pub struct CoincidenceData {
	i: (usize, usize),
	len: usize,
}

pub struct Coincidences<'a> {
	coincidences: &'a CoincidencesData,
	m_offset: usize,
	m_text: &'a [Alph],
}

pub struct Coincidence<'a> {
	coincidence: &'a CoincidenceData,
	m_text: &'a [Alph],
}

pub struct CoincidencesTableIter<'a> {
	it: iter::Enumerate<<&'a Vec<CoincidencesData> as IntoIterator>::IntoIter>,
	m_text: &'a[Alph],
}

pub struct CoincidencesIter<'a> {
	it: <&'a Vec<CoincidenceData> as IntoIterator>::IntoIter,
	m_text: &'a[Alph],
}

impl<'a> Coincidences<'a> {
	pub fn offset(&self) -> usize {
		self.m_offset
	}
}

impl<'a> iter::IntoIterator for &'a CoincidencesTable<'a> {
	type Item = Coincidences<'a>;
	type IntoIter = CoincidencesTableIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		CoincidencesTableIter {
			it: self.coincidences.iter().enumerate(),
			m_text: self.m_text,
		}
	}
}

impl<'a> iter::IntoIterator for &'a Coincidences<'a> {
	type Item = Coincidence<'a>;
	type IntoIter = CoincidencesIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		CoincidencesIter {
			it: self.coincidences.coincidences.iter(),
			m_text: self.m_text,
		}
	}
}

impl<'a> iter::Iterator for CoincidencesTableIter<'a> {
	type Item = Coincidences<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.it.next() {
			Some((offset, coincidences)) => {
				Some(Coincidences {
					coincidences,
					m_offset: offset,
					m_text: self.m_text,
				})
			},
			None => None,
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
					m_text: self.m_text,
				})
			},
			None => None,
		}
	}
}

impl<'a> CoincidencesTable<'a> {
	pub fn new(text: &'a [Alph], n: usize) -> Self {
		let mut cs = CoincidencesTable {
			coincidences: Vec::new(),
			m_text: text,
		};

		for i in 1..cmp::min(n,text.len()) {
			cs.coincidences.push(CoincidencesData::new(text, i));
		}
		cs
	}
}

impl CoincidencesData {
	fn new(text: &[Alph], offset: usize) -> CoincidencesData {
		let mut c = CoincidencesData {
			coincidences: Vec::new(),
		};
		let mut push = |streak: Option<(usize, usize)>| {
			if let Some((i, j)) = streak {
				let len = j - i + 1;
				if len >= 2 {
					c.coincidences.push(CoincidenceData::new((i, i + offset), len));
				}
			}
		};

		let mut streak: Option<(usize, usize)> = None;
		let iter_a = text.iter();
		let iter_b = iter_a.clone().skip(offset);
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
		c.coincidences.sort_by(|a, b| {
			a.len.cmp(&b.len)
		});
		c
	}
}

impl CoincidenceData {
	fn new(i: (usize, usize), len: usize) -> Self {
		CoincidenceData {
			i,
			len,
		}
	}
}

impl<'a> Coincidences<'a> {
	pub fn get_offset(&self) -> usize {
		self.m_offset
	}

}

impl<'a> Coincidence<'a> {
	pub fn indices(&self) -> (usize, usize) {
		(self.coincidence.i.0, self.coincidence.i.1)
	}

	pub fn len(&self) -> usize {
		self.coincidence.len
	}

	pub fn text(&self) -> &'a [Alph] {
		self.m_text
	}
}
