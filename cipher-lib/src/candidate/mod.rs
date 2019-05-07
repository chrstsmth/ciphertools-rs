use std::fmt;
use crate::key::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate<K> where
	K: Key,
{
	pub score: u32,
	pub key: K,
	pub text: String,
}

impl<K> fmt::Display for Candidate<K> where
	K: Key,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.score, self.key, self.text)
	}
}
