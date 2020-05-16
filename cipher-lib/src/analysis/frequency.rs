use enum_map::*;
use crate::language_model::*;
use crate::alphabet::latin::*;
use std::iter::*;
use std::ops::Index;

pub struct Frequencies(EnumMap<Latin,u32>);

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
