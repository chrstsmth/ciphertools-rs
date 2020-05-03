use std::fmt;
use crate::key::Key;
use crate::key::any_key::AnyKey;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate {
	score: u32,
	key: AnyKey,
	text: String,
}


#[derive(Clone)]
pub struct Candidates(Vec<Candidate>);

impl Candidate {
	pub fn new<K: Key>(score: u32, key: K, text: String) -> Self {
		Candidate {
			score,
			key: K::into(key),
			text,
		}
	}

	pub fn key(&self) -> AnyKey {
		return self.key.clone();
	}

	pub fn score(&self) -> u32 {
		return self.score;
	}
}

impl fmt::Display for Candidate
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.score, self.key, self.text)
	}
}

pub struct CandidatesIntermediates<I> where
	I: Iterator<Item = Candidate>
{
	it: I,
	cs: Candidates,
}

impl<I> Iterator for CandidatesIntermediates<I> where
	I: Iterator<Item = Candidate>
{
	type Item = Vec<Candidate>;

	fn next(&mut self) -> Option<Self::Item> {
		for c in &mut self.it {
			if self.cs.present_candidate(&c) {
				return Some(self.cs.0.clone());
			}
		}
		return None;
	}
}

impl Candidates {
	fn with_length(n: usize) -> Candidates
	{
		Candidates(Vec::with_capacity(n))
	}


	pub fn intermediates<I>(n: usize, it: I) -> CandidatesIntermediates<I> where
		I: Iterator<Item = Candidate>
	{
		CandidatesIntermediates {
			it,
			cs: Candidates::with_length(n)
		}
	}

	pub fn present_candidate(&mut self, c: &Candidate) -> bool
	{
		if self.0.len() < self.0.capacity() {
			self.0.push(c.clone());
		} else if self.0.last().unwrap() < c && !self.0.contains(&c) {
				self.0.pop();
				self.0.push(c.clone());
		} else {
			return false;
		}

		self.0.sort_by(|a, b| b.cmp(a)); // Reverse order sort
		return true;
	}
}

impl fmt::Display for Candidates
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for c in &self.0 {
			writeln!(f, "{}", *c)?
		}
		Ok(())
	}
}
