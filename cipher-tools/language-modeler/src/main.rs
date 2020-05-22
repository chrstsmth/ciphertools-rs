extern crate cipher_lib;
extern crate serde;
extern crate serde_json;
extern crate clap;

use std::convert::TryFrom;
use std::fs::File;
use cipher_lib::language_model::*;
use cipher_lib::alphabet::latin::*;
use clap::{App, Arg, AppSettings};
use std::io::BufReader;
use std::io::BufRead;

fn main() {
	let mut l: LanguageModel = LanguageModel::new();

	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.arg(Arg::with_name("windowed")
			.long("windowed"))
		.arg(Arg::with_name("files")
			.value_name("FILES")
			.required(true))
		.get_matches();

	let files = matches.values_of("files").unwrap();
	for file_name in files {
		let file = match File::open(&file_name ) {
			Err(why) => panic!("couldn't open {}: {}", file_name, why.to_string()),
			Ok(file) => file,
		};

		let windowed = matches.is_present("windowed");
		let it = BufReader::new(file)
			.lines()
			.map(|y| {
				let y = match y {
					Err(why) => panic!("Failed to read {}: {}", file_name, why.to_string()),
					Ok(y) => y
				};
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
				true
			})
			.filter(|(_, freq)| {
				if *freq < 50 {
					return true //TODO
				}
				true
			})
			.map(|(word, freq)| {
				let word: Vec<Latin> = word.chars().map(|x| Latin::try_from(x).unwrap()).collect();
				(word, freq)
			});

		for (word, freq) in it {
			if windowed {
				l.windowed_insert_word_n_times(word.clone().into_iter(), freq);
			} else {
				l.insert_word_n_times(word.clone().into_iter(), freq);
			}
		}

		println!("{}", serde_json::to_string(&l).unwrap());
	}
}
