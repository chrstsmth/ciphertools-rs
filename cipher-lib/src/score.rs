use std::convert::TryFrom;

use crate::language_model::*;
use crate::alphabet::latin::*;

pub fn score<S>(tr: LanguageModelTraverser, mut s: S) -> u32 where
	S: Iterator<Item = Latin> + Clone,
{
	let mut score: u32 = 0;
	loop {
		let text = s.clone();
		let mut cursor = tr.clone();

		for (i, c) in text.enumerate() {
			let next = cursor.next(c);
			match next {
				None => break,
				Some(node) => {
					score += (u32::try_from(i).unwrap() + 1) * node.freq();
				}
			}
		}

		if s.next().is_none() {
			break;
		}
	}
	score
}
