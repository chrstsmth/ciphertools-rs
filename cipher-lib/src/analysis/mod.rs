pub mod coincidence_count;

use enum_map::*;
use crate::language_model::LanguageModel;
use crate::pallet::alph::*;
use std::collections::HashMap;

pub fn frequency<E>(text: &[E]) -> EnumMap<E,u32> where
	E: Enum<u32> + Copy,
{
	let mut frequency = enum_map!{ _ => 0 };
	for c in text {
		frequency[*c] += 1;
	}
	frequency
}

pub fn frequency_ngram<E>(text: &[E], length: usize) -> HashMap<Vec<E>, u32> where
	E: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
	let mut frequency = HashMap::new();
	for w in text.windows(length) {
		*frequency.entry(Vec::from(w)).or_insert(0) += 1
	}
	frequency
}

pub fn frequency_language(language: LanguageModel) -> EnumMap<Alph,u32> where
{
	EnumMap::from(|e|
		match language.traverse().next(e) {
			Some(node) => node.freq(),
			None => 0,
		})
}

pub fn distribution<E>(frequency: EnumMap<E,u32>) -> EnumMap<E,f64> where
	E: Enum<u32> + Enum<f64> + Copy,
{
	let sum: u32 = frequency.iter()
		.map(|(_,i)| i)
		.sum();
	let distribution = EnumMap::from(|e| f64::from(frequency[e]) / f64::from(sum));

	distribution
}

pub fn distribution_ngram<E>(frequency: HashMap<Vec<E>,u32>) -> HashMap<Vec<E>,f64> where
	E: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
	let mut sum: u32 = 0;
	for (_, i) in &frequency {
		sum += i;
	}

	frequency.into_iter().map(|(k, v)| (k, f64::from(v) / f64::from(sum))).collect()
}

pub fn chi_squared<E>(text: &EnumMap<E,f64>, lang: &EnumMap<E,f64>) -> f64 where
	E: Enum<f64>,
{
	let mut chi_squared = 0.0;
	for ((_,c),(_,e)) in text.iter().zip(lang) {
		let ce = *c - *e;
		chi_squared += ce * ce / *e;
	}
	chi_squared
}
