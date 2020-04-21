use crate::cipher::*;
use crate::key::*;
use crate::candidate::*;
use std::str::Chars;

pub fn dictionary_attack<C,Dict,Can,Exit,Score>(ciphertext: &str, dict: Dict, config: &C::Config, score: Score, mut candidates: Can, exit: Exit)
where
	C: Cipher,
	Dict: Iterator<Item = C::Key>,
	Can: FnMut(&Candidate<C>),
	Exit: Fn() -> bool,
	Score: Fn(Chars) -> u32,
{
	for key in dict {
		let text = C::decipher(&ciphertext, &key, &config);

		let can = Candidate {
			score: score(text.chars()),
			text: text,
			key: key,
		};

		candidates(&can);

		if exit() {
			break;
		}
	}
}

pub fn hill_climb<C,Dict,Can,Exit,Score>(ciphertext: &str, dict: Dict, config: &C::Config, score: Score, mut candidates: Can, exit: Exit)
where
	C: Cipher,
	C::Key: IntoMutationIterator,
	Dict: Iterator<Item = C::Key>,
	Can: FnMut(&Candidate<C>),
	Exit: Fn() -> bool,
	Score: Fn(Chars) -> u32,
{
	for key in dict {
		let text = C::decipher(&ciphertext, &key, &config);

		let mut best_mutation = Candidate {
			score: score(text.chars()),
			text: text,
			key: key.clone(),
		};
		candidates(&best_mutation);

		let mut climbed = true;
		while climbed {
			climbed = false;

			for mutated_key in key.clone().into_mutation_iterator() {
				let text = C::decipher(&ciphertext, &mutated_key, &config);

				let competitor = Candidate {
					score: score(text.chars()),
					text: text,
					key: mutated_key.clone(),
				};
				if competitor > best_mutation {
					best_mutation = competitor;
					climbed = true;
				}

				candidates(&best_mutation);

				if exit() {
					return;
				}
			}
		}
	}
}
