use std::cmp;

pub struct Coincidences<'a, A: Eq> (CoincidencesAllOffets<'a, A>);

pub struct CoincidencesAllOffets<'a, A>
where
	A: Eq
{
	c: Vec<CoincidencesAtOffset<'a, A>>,
}

pub struct CoincidencesAtOffset<'a, A>
where
	A: Eq
{
	c: Vec<Coincidence<'a, A>>,
}

pub struct Coincidence<'a, A>
where
	A: Eq
{
	c: &'a [A],
	i: (usize, usize),
	len: usize,
}

impl<'a, A> Coincidences<'a, A>
where
	A: Eq
{
	pub fn with_length(n: usize, text: &'a [A]) -> Self {
		Coincidences(CoincidencesAllOffets::with_length(n, text))
	}

	pub fn all_offsets(&'a self) -> &'a CoincidencesAllOffets<'a, A> {
		&self.0
	}
}

impl<'a, A> CoincidencesAllOffets<'a, A>
where
	A: Eq
{
	fn with_length(n: usize, text: &'a [A]) -> Self {
		let mut all = CoincidencesAllOffets {
			c: Vec::new(),
		};

		for n in 1..cmp::min(n + 1, text.len() / 2) {
			all.c.push(CoincidencesAtOffset::with_offset(text, n));
		}
		all
	}
}

impl<'a, A> IntoIterator for &'a CoincidencesAllOffets<'a, A>
where
	A: Eq
{
    type Item = &'a CoincidencesAtOffset<'a, A>;
    type IntoIter = std::slice::Iter<'a, CoincidencesAtOffset<'a, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.c.iter()
    }
}

impl<'a, A> CoincidencesAtOffset<'a, A>
where
	A: Eq
{
	fn with_offset(text: &'a [A], n: usize) -> Self {
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

impl<'a, A> IntoIterator for &'a CoincidencesAtOffset<'a, A>
where
	A: Eq
{
    type Item = &'a Coincidence<'a, A>;
    type IntoIter = std::slice::Iter<'a, Coincidence<'a, A>>;

    fn into_iter(self) -> Self::IntoIter {
        self.c.iter()
    }
}

impl<'a, A> Coincidence<'a, A>
where
	A: Eq
{
	pub fn text(&self) -> &'a [A] {
		self.c
	}

	pub fn indices(&self) -> (usize, usize) {
		self.i
	}

	pub fn len(&self) -> usize {
		self.len
	}
}
