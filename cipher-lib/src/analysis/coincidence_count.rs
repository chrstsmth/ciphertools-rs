use crate::pallet::alph::*;
use std::cmp;
use std::fmt;

pub struct Coincidences<'a>(CoincidencesAllOffets<'a>);

pub struct CoincidencesAllOffets<'a> {
	c: Vec<CoincidencesAtOffset<'a>>,
}

pub struct CoincidencesAtOffset<'a> {
	c: Vec<Coincidence<'a>>,
}

pub struct Coincidence<'a> {
	c: &'a [Alph],
	i: (usize, usize),
	len: usize,
}

impl<'a> Coincidences<'a> {
	pub fn with_length(n: usize, text: &'a [Alph]) -> Self {
		Coincidences(CoincidencesAllOffets::with_length(n, text))
	}
}

impl<'a> fmt::Display for Coincidences<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		for (i, cs) in self.0.into_iter().enumerate() {
			write!(f, "{}:", i + 1)?;
			for c in cs {
				write!(f, " {}", c.text().into_iter().map(|a| char::from(*a)).collect::<String>())?;
			}
			writeln!(f, "")?;
		}
		Ok(())
	}
}

impl<'a> CoincidencesAllOffets<'a> {
	fn with_length(n: usize, text: &'a [Alph]) -> Self {
		let mut all = CoincidencesAllOffets {
			c: Vec::new(),
		};

		for n in 1..cmp::min(n + 1, text.len() / 2) {
			all.c.push(CoincidencesAtOffset::with_offset(text, n));
		}
		all
	}
}

impl<'a> IntoIterator for &'a CoincidencesAllOffets<'a> {
    type Item = &'a CoincidencesAtOffset<'a>;
    type IntoIter = std::slice::Iter<'a, CoincidencesAtOffset<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.c.iter()
    }
}

impl<'a> CoincidencesAtOffset<'a> {
	fn with_offset(text: &'a [Alph], n: usize) -> Self {
		let mut c = CoincidencesAtOffset {
			c: Vec::new(),
		};

		let mut push = |streak: Option<(usize, usize)>| {
			if let Some((i, j)) = streak {
				let len = j - i + 1;
				if len >= 3 {
					c.c.push(Coincidence {
						c: text.get(i..j+1).unwrap(),
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
		c
	}
}

impl<'a> IntoIterator for &'a CoincidencesAtOffset<'a> {
    type Item = &'a Coincidence<'a>;
    type IntoIter = std::slice::Iter<'a, Coincidence<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.c.iter()
    }
}

impl<'a> Coincidence<'a> {
	pub fn text(&self) -> &'a [Alph] {
		self.c
	}

	pub fn indices(&self) -> (usize, usize) {
		self.i
	}

	pub fn len(&self) -> usize {
		self.len
	}
}
