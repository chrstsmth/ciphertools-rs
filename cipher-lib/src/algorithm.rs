use std::convert::TryFrom;
use crate::cipher::*;
use crate::key::*;
use crate::key::any_key::*;
use crate::candidate::*;
use std::str::Chars;

pub fn dictionary_attack<C,Dict,Score>(ciphertext: &str, dict: Dict, config: C::Config, score: Score) -> impl Iterator<Item = Candidate>
where
	C: Cipher,
	Dict: Iterator<Item = C::Key>,
	Score: Fn(Chars) -> u32,
{
	let ciphertext_clone = String::from(ciphertext);
	dict.map(move |key| {
		let text = C::decipher(&ciphertext_clone, &key, &config);
		Candidate::new(score(text.chars()), key, text)
	})
}

struct HillClimbIterator<C,Score>
where
	C: Cipher,
	C::Key: IntoMutationIterator,
	Score: Fn(Chars) -> u32,
{
	ciphertext: String,
	best: Candidate,
	config: C::Config,
	score: Score
}

impl<C,Score> HillClimbIterator<C,Score>
where
	C: Cipher,
	C::Key: IntoMutationIterator,
	Score: Fn(Chars) -> u32,
{
	fn new(ciphertext: String, seed: C::Key, config: C::Config, score: Score) -> Self {
		let plaintext = C::decipher(&ciphertext, &seed, &config);

		HillClimbIterator {
			ciphertext,
			best: Candidate::new((score)(plaintext.chars()), seed, plaintext),
			config,
			score
		}
	}
}

impl<C,Score> Iterator for HillClimbIterator<C,Score>
where
	C: Cipher,
	C::Key: IntoMutationIterator + TryFrom<AnyKey>,
	Score: Fn(Chars) -> u32,
	<C::Key as TryFrom<AnyKey>>::Error: std::fmt::Debug, // for unrap()
{
	type Item = Candidate;
	fn next(&mut self) -> Option<Self::Item> {
		let mut climbed = false;

		let seed = C::Key::try_from(self.best.key()).unwrap();

		for k in seed.into_mutation_iterator() {
			let plaintext = C::decipher(&self.ciphertext, &k, &self.config);
			let score = (self.score)(plaintext.chars());

			if score > self.best.score() {
				self.best = Candidate::new(score, k, plaintext);
				climbed = true;
			}
		}

		if climbed {
			Some(self.best.clone())
		} else {
			None
		}
	}
}

pub fn hill_climb<C,Score>(ciphertext: &str, seed_key: C::Key, config: &C::Config, score: Score) -> impl Iterator<Item = Candidate>
where
	C: Cipher,
	C::Key: IntoMutationIterator,
	Score: Fn(Chars) -> u32,
	<C::Key as TryFrom<AnyKey>>::Error: std::fmt::Debug, // for unrap()
{
	HillClimbIterator::<C,_>::new(
		String::from(ciphertext),
		seed_key,
		config.clone(),
		score)
}
