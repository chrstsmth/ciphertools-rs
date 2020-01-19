extern crate serde;
extern crate serde_json;
extern crate cipher_lib;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::convert::TryFrom;

use cipher_lib::language_model::*;
use cipher_lib::pallet::alph::*;

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
			.map(|x| Alph::try_from(x))
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap());
		l.insert_words(&mut i, 5);
		println!("{}", serde_json::to_string(&l).unwrap());
	}
}
