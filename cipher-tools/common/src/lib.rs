extern crate cipher_lib;
extern crate clap;

pub mod cli;
pub mod parse;

use cipher_lib::pallet::alph::*;
use std::convert::TryFrom;

pub fn string_to_alph(s: String) -> Vec<Alph> {
	s.chars()
		.map(|c| Alph::try_from(c))
		.filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect()
}
