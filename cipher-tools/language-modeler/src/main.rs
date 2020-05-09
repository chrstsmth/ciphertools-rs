extern crate cipher_lib;
extern crate serde;
extern crate serde_json;

use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;

use cipher_lib::language_model::*;
use cipher_lib::alphabet::latin::*;

fn main() {
	let mut l: LanguageModel = LanguageModel::new();

	for arg in std::env::args().skip(1) {
		let mut file = match File::open(&arg) {
			Err(why) => panic!("couldn't open {}: {}", arg, why.to_string()),
			Ok(file) => file,
		};

		let mut s = String::new();
		match file.read_to_string(&mut s) {
			Err(why) => panic!("couldn't open {}: {}", arg, why.to_string()),
			_ => (),
		}

		let it = s
			.lines()
			.map(|y| {
				let mut x = y.split('\t');
				x.next(); // word number
				let word = String::from(x.next().unwrap());
				x.next(); // repeated word
				let freq: u32 = x.next().unwrap().parse().unwrap();
				(word, freq)
			})
			.filter(|(word, _)| {
				for c in word.chars() {
					if Latin::try_from(c).is_err() {
						return false;
					}
				}
				return true;
			})
			.map(|(word, freq)| {
				let word: Vec<Latin> = word.chars().map(|x| Latin::try_from(x).unwrap()).collect();
				(word, freq)
			});

		for (word, freq) in it {
			l.insert_word_n_times(&mut word.clone().into_iter(), freq);
		}

		println!("{}", serde_json::to_string(&l).unwrap());
	}
}
