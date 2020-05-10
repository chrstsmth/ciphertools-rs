pub mod coincidence_count;

use enum_map::*;
use crate::language_model::*;
use crate::alphabet::latin::*;
use std::iter::*;
use std::ops::Index;

pub struct Frequencies(EnumMap<Latin,u32>);
pub struct Distribution(EnumMap<Latin,f64>);

impl Frequencies {
	pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Latin, &'a u32)> {
		self.0.iter()
	}
}

impl IntoIterator for Frequencies {
	type Item = (Latin, u32);
	type IntoIter = <EnumMap::<Latin,u32> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl Index<Latin> for Frequencies {
	type Output = u32;
	fn index(&self, i: Latin) -> &Self::Output {
		&self.0[i]
	}
}

impl Index<Latin> for Distribution {
	type Output = f64;
	fn index(&self, i: Latin) -> &Self::Output {
		&self.0[i]
	}
}

impl Distribution {
	pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Latin, &'a f64)> {
		self.0.iter()
	}
}

impl IntoIterator for Distribution {
	type Item = (Latin, f64);
	type IntoIter = <EnumMap::<Latin,f64> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

pub fn frequency(text: &[Latin]) -> Frequencies {
	let mut frequency = enum_map!{ _ => 0 };
	for c in text {
		frequency[*c] += 1;
	}
	Frequencies(frequency)
}

pub fn frequency_language(language: &LanguageModel) -> Frequencies {
	Frequencies(EnumMap::from(|e|
		match language.traverse().next(e) {
			Some(node) => node.freq(),
			None => 0,
		}))
}

pub fn distribution(frequency: Frequencies) -> Distribution {
	let sum: u32 = frequency.iter()
		.map(|(_,i)| i)
		.sum();

	Distribution(EnumMap::from(|e| f64::from(frequency[e]) / f64::from(sum)))
}

pub fn chi_squared(text: &EnumMap<Latin,f64>, lang: &EnumMap<Latin,f64>) -> f64 {
	let mut chi_squared = 0.0;
	for ((_,c),(_,e)) in text.iter().zip(lang) {
		let ce = *c - *e;
		chi_squared += ce * ce / *e;
	}
	chi_squared
}

pub fn index_of_coincidence(distribution: Distribution) -> f64
{
	let mut ic = 0.0;
	for a in Latin::iter() {
		let prob = distribution[a];
		ic += prob * prob;
	}
	ic
}

pub fn measure_of_roughness(distribution: Distribution) -> f64
{
	let ic = index_of_coincidence(distribution);
	let mut roughness = ic;
	roughness -= 2.0 / f64::from(Latin::LENGTH);
	roughness += 1.0 / f64::from(Latin::LENGTH);
	roughness
}
