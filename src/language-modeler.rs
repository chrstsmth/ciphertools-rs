#![feature(nll)]

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::convert::TryFrom;

mod language_model;
mod cipher;
mod try_from_err;

use language_model::*;
use cipher::pallet::alph::*;

fn main() {
	let mut l: LanguageModel = LanguageModel::new();

	for arg in std::env::args().skip(1) {

		let mut file =  match File::open(&arg) {
			Err(why) => panic!("couldn't open {}: {}", arg, why.description()),
			Ok(file) => file,
		};

		let mut s = String::new();
		match file.read_to_string(&mut s) {
			Err(why) => panic!("couldn't open {}: {}", arg, why.description()),
			_ => (),
		}

		let mut i = s.chars()
			.filter(|x| *x != '\n')
			.map(|x| Alph::try_from(x).unwrap());
		l.insert_words(&mut i, 5);
		println!("{}", serde_json::to_string(&l).unwrap());
	}
}
