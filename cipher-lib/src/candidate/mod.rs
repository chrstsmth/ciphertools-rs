use crate::key::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate<K> where
	K: Key,
{
	pub score: u32,
	pub key: K,
	pub text: String,
}
