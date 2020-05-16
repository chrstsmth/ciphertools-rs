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

impl<S> From<S> for Frequencies
where
	S: Iterator<Item = Latin>
{
	fn from(text: S) -> Self {
		let mut frequency = enum_map!{ _ => 0 };
		for c in text {
			frequency[c] += 1;
		}
		Frequencies(frequency)
	}
}

impl From<&LanguageModel> for Frequencies {
	fn from(language: &LanguageModel) -> Self {
		Frequencies(EnumMap::from(|e|
			match language.traverse().next(e) {
				Some(node) => node.freq(),
				None => 0,
			}))
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

impl Distribution {
	pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Latin, &'a f64)> {
		self.0.iter()
	}
}

impl From<Frequencies> for Distribution {
	fn from(frequency: Frequencies) -> Self {
		let sum: u32 = frequency.iter()
			.map(|(_,i)| i)
			.sum();

		Distribution(EnumMap::from(|e| f64::from(frequency[e]) / f64::from(sum)))
	}
}

impl From<&[(Latin, f64)]> for Distribution {
	fn from(data: &[(Latin, f64)]) -> Distribution  {
		let mut distribution = enum_map!{ _ => 0.0 };
		for (l, d) in data {
			distribution[*l] = *d;
		}
		Distribution(distribution)
	}
}

impl Index<Latin> for Distribution {
	type Output = f64;
	fn index(&self, i: Latin) -> &Self::Output {
		&self.0[i]
	}
}

impl IntoIterator for Distribution {
	type Item = (Latin, f64);
	type IntoIter = <EnumMap::<Latin,f64> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

pub fn chi_squared(text: Distribution, lang: Distribution) -> f64 {
	let mut chi_squared = 0.0;
	for ((_,c),(_,e)) in text.iter().zip(lang) {
		let ce = *c - e;
		chi_squared += ce * ce / e;
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
