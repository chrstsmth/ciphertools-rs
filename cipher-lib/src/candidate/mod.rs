use std::fmt;
use crate::key::*;
use min_max_heap::*;
use std::sync::Mutex;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate<K> where
	K: Key, //TODO cange to cipher?
{
	pub score: u32,
	pub key: K,
	pub text: String,
}

pub struct Candidates<K: Key>
{
	candidates: Mutex<MinMaxHeap<Candidate<K>>>,
}

pub trait Model<K: Key>
{
	fn insert_candidate(&mut self, candidate: Candidate<K>);
}

impl<K: Key> Model<K> for Candidates<K> {
	fn insert_candidate(&mut self, candidate: Candidate<K>)
	{
		let mut candidates = self.candidates.lock().unwrap();

		if candidates.len() < candidates.capacity() {
			candidates.push(candidate);
		} else if *candidates.peek_min().unwrap() < candidate {
			candidates.replace_min(candidate);
		}
	}
}

impl<K> fmt::Display for Candidate<K> where
	K: Key,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.score, self.key, self.text)
	}
}

impl<K: Key> Candidates<K> {
	pub fn with_capacity(cap: usize) -> Candidates<K> {
		Candidates::<K> {
			candidates: Mutex::new(MinMaxHeap::<Candidate<K>>::with_capacity(cap)),
		}
	}

	pub fn into_vec(self) -> Vec<Candidate<K>>
	{
		let a = self.candidates.into_inner().unwrap();
		a.into_vec_desc()
	}
}
