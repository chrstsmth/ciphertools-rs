use crate::alphabet::latin::*;
use std::iter::*;
use crate::analysis::distribution::*;

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
