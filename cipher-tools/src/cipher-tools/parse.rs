use std::fs::File;
use std::process;

use cipher_lib::language_model::*;
use cipher_lib::cipher::*;
use cipher_lib::cipher::caesar::*;
use cipher_lib::cipher::vigenere::*;
use std::io::prelude::*;
use std::io::*;
use std::error::Error;
use std::str::FromStr;

pub trait DictionaryOption: Cipher {
	fn dictionary_option(matches: &clap::ArgMatches) -> Box<dyn Iterator<Item = Self::Key>>;
}

impl DictionaryOption for Caesar {
	fn dictionary_option(matches: &clap::ArgMatches) -> Box<dyn Iterator<Item = Self::Key>> {
		let dict_file = matches.value_of("dict-file").unwrap();
		dictionary_file::<Self>(&dict_file)
	}
}

impl DictionaryOption for Vigenere {
	fn dictionary_option(matches: &clap::ArgMatches) -> Box<dyn Iterator<Item = Self::Key>> {
		let dict_file = matches.value_of("dict-file").unwrap();
		dictionary_file::<Self>(&dict_file)
	}
}

pub fn language_model_option(matches: &clap::ArgMatches) -> LanguageModel
{
	language_model(matches.value_of("language").unwrap())
}

pub fn ciphertext_option(matches: &clap::ArgMatches) -> String
{
	text(matches.value_of("ciphertext").unwrap())
}

pub fn plaintext_option(matches: &clap::ArgMatches) -> String
{
	text(matches.value_of("plaintext").unwrap())
}

pub fn key_option<C: Cipher>(matches: &clap::ArgMatches) -> C::Key
{
	key::<C>(matches.value_of("key").unwrap())
}

pub fn dictionary_file<C: Cipher>(filename: &str) -> Box<dyn Iterator<Item = C::Key>>
{

	let file = match File::open(filename) {
		Err(why) => {
			eprintln!("{}: {}", filename, why);
			process::exit(1);
		}
		Ok(file) => file,
	};

	//Must explicitly clone into closure
	//https://github.com/rust-lang/rfcs/issues/2407
	let filename_clone = String::from(filename);

	let dict = BufReader::new(file)
		.lines()
		.map(|x| x.unwrap_or_else(|e|
			{
				eprintln!("{}", e.description());
				process::exit(1);
			})
			)
		.enumerate()
		.map(move |x| {
			let (num, line) = x;
			let line_num = num + 1;
			match C::Key::from_str(line.as_str()) {
				Err(_) => {
					eprintln!("{}:{}: failed to parse \"{}\"", filename_clone, line_num, line);
					process::exit(1);
				}
				Ok(key) => key
			}
			});

	Box::new(dict)
}

pub fn language_model(filename: &str) -> LanguageModel
{
	let file = match File::open(filename) {
		Err(why) => {
			eprintln!("{}: {}", filename, why);
			process::exit(1);
		}
		Ok(file) => file,
	};

	let language = match serde_json::from_reader(BufReader::new(file)) {
		Err(why) => {
			eprintln!("{}: {}", filename, why);
			process::exit(1);
		}
		Ok(language) => language,
	};

	language
}

pub fn key<'a, C: Cipher>(key_str: &str) -> C::Key
{
	match C::Key::from_str(key_str) {
		Ok(key) => key,
		_ => {
			println!("{}: Parse key failed", key_str);
			process::exit(1);
		}
	}
}

fn text<'a>(filename: &str) -> String
{
	let file = match File::open(&filename) {
		Err(why) => {
			eprintln!("{}: {}", filename, why);
			process::exit(1);
		}
		Ok(file) => file,
	};

	let mut reader = BufReader::new(file);

	let mut text = String::new();
	match reader.read_to_string(&mut text) {
		Err(why) => {
			eprintln!("{}: {}", filename, why);
			process::exit(1);
		},
		Ok(_) => (),
	}

	text
}

