use std::fmt;
use crate::cipher::*;
use min_max_heap::*;
use std::sync::Mutex;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate<C> where
	C: Cipher,
{
	pub score: u32,
	pub key: C::Key,
	pub text: String,
}

pub struct Candidates<C: Cipher>
{
	pub candidates: Mutex<MinMaxHeap<Candidate<C>>>,
}

impl<C: Cipher> fmt::Display for Candidate<C>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.score, self.key, self.text)
	}
}

impl<C: Cipher> Candidates<C> {
	pub fn with_capacity(cap: usize) -> Candidates<C>
	{
		Candidates::<C> {
			candidates: Mutex::new(MinMaxHeap::<Candidate<C>>::with_capacity(cap)),
		}
	}

	pub fn insert_candidate(&mut self, candidate: Candidate<C>)
	{
		let mut candidates = self.candidates.lock().unwrap();

		if candidates.len() < candidates.capacity() {
			candidates.push(candidate);
		} else if *candidates.peek_min().unwrap() < candidate {
			candidates.replace_min(candidate);
		}
	}
}

impl<C: Cipher> fmt::Display for Candidates<C>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for candidate in &(*self.candidates.lock().unwrap()) {
			writeln!(f, "{}", candidate)?
		}
		Ok(())
	}
}
