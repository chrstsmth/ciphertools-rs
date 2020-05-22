use enum_map::*;
use crate::alphabet::latin::*;
use std::iter::*;
use std::ops::Index;
use crate::analysis::frequency::*;

pub struct Distribution(EnumMap<Latin,f64>);

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

