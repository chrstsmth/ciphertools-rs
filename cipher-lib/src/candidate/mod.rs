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
	candidates: Mutex<MinMaxHeap<Candidate<C>>>,
}

pub trait Model<C: Cipher>
{
	fn insert_candidate(&mut self, candidate: Candidate<C>);
}

impl<C: Cipher> Model<C> for Candidates<C> {
	fn insert_candidate(&mut self, candidate: Candidate<C>)
	{
		let mut candidates = self.candidates.lock().unwrap();

		if candidates.len() < candidates.capacity() {
			candidates.push(candidate);
		} else if *candidates.peek_min().unwrap() < candidate {
			candidates.replace_min(candidate);
		}
	}
}

impl<C: Cipher> fmt::Display for Candidate<C> where
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.score, self.key, self.text)
	}
}

impl<C: Cipher> Candidates<C> {
	pub fn with_capacity(cap: usize) -> Candidates<C> {
		Candidates::<C> {
			candidates: Mutex::new(MinMaxHeap::<Candidate<C>>::with_capacity(cap)),
		}
	}

	pub fn into_vec(self) -> Vec<Candidate<C>>
	{
		let a = self.candidates.into_inner().unwrap();
		a.into_vec_desc()
	}
}
