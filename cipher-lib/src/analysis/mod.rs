pub mod coincidence_count;

use enum_map::*;
use crate::language_model::*;
use crate::pallet::alph::*;
use std::collections::HashMap;
use std::iter::*;

pub fn frequency(text: &[Alph]) -> EnumMap<Alph,u32> {
	let mut frequency = enum_map!{ _ => 0 };
	for c in text {
		frequency[*c] += 1;
	}
	frequency
}

pub fn frequency_language(language: &LanguageModel) -> EnumMap<Alph,u32> {
	EnumMap::from(|e|
		match language.traverse().next(e) {
			Some(node) => node.freq(),
			None => 0,
		})
}

pub fn ngram_frequency(length: usize, text: &[Alph]) -> HashMap<Vec<Alph>, u32>
{
	let mut frequency = HashMap::new();
	for w in text.windows(length) {
		*frequency.entry(Vec::from(w)).or_insert(0) += 1
	}
	frequency
}

pub fn ngram_frequency_language(length: usize, language: &LanguageModel) -> HashMap<Vec<Alph>,u32> {
	#[derive(Clone)]
	struct Iter<'a> {
		a: Alph,
		it: AlphIterator,
		tr: LanguageModelTraverser<'a>,
	};

	let mut frequency = HashMap::new();

	if length == 0 {
		return frequency;
	}

	let mut stack = Vec::with_capacity(length);
	stack.push(
		Iter {
			a: Alph::A,
			it: Alph::iter(),
			tr: language.traverse(),
		});

	while !stack.is_empty() {
		let tail = stack.last_mut().unwrap();
		let a = tail.it.next();
		match a {
			Some(a) => {
				tail.a = a;
				let mut next_tr: LanguageModelTraverser = tail.tr.clone();
				match next_tr.next(a) {
					Some(node) => {
						if stack.len() == length {
							frequency.insert( Vec::from_iter(
									stack.iter().map(|it| it.a)), node.freq());
						} else {
							stack.push(Iter{
								a: Alph::A,
								it: Alph::iter(),
								tr: next_tr,
							});
						}
					},
					None => {
						tail.it.next();
					}
				}
			}
			None => {
				stack.pop();
			}
		}
	}

	frequency
}

pub fn distribution(frequency: EnumMap<Alph,u32>) -> EnumMap<Alph,f64> {
	let sum: u32 = frequency.iter()
		.map(|(_,i)| i)
		.sum();

	EnumMap::from(|e| f64::from(frequency[e]) / f64::from(sum))
}

pub fn ngram_distribution(frequency: HashMap<Vec<Alph>,u32>) -> HashMap<Vec<Alph>,f64> {
	let mut sum: u32 = 0;
	for (_, i) in &frequency {
		sum += i;
	}

	frequency.into_iter().map(|(k, v)| (k, f64::from(v) / f64::from(sum))).collect()
}

pub fn chi_squared(text: &EnumMap<Alph,f64>, lang: &EnumMap<Alph,f64>) -> f64 {
	let mut chi_squared = 0.0;
	for ((_,c),(_,e)) in text.iter().zip(lang) {
		let ce = *c - *e;
		chi_squared += ce * ce / *e;
	}
	chi_squared
}
