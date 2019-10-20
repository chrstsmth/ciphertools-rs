use std::cmp;
use std::collections::HashMap;

pub fn frequency_analysis_length<E, C>(text: &[E], length: usize, mut insert: C) where
	E: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
	C: FnMut(&[E]),
{
	for w in text.windows(length) {
		insert(w);
	}
}

pub fn frequency_analysis<E>(text: &[E]) -> HashMap<Vec<E>, i32> where
	E: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
	let mut frequencies: HashMap<Vec<E>, i32> = HashMap::new();
	for n in 1..cmp::min(text.len(), 10) {
		frequency_analysis_length(text, n, |w| *frequencies.entry(Vec::from(w)).or_insert(0) += 1);
	}
	frequencies
}

