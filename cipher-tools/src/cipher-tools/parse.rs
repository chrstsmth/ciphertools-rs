use clap::ArgMatches;
use std::fs::File;
use std::process;

use cipher_lib::language_model::*;
use cipher_lib::cipher::*;
use std::io::prelude::*;
use std::io::*;
use std::error::Error;
use std::str::FromStr;

pub struct Arguments<C: Cipher> {
	pub language_model: Option<LanguageModel>,
	pub dictionary: Option<Box<dyn Iterator<Item = C::Key>>>,
	pub ciphertext: Option<String>,
	pub plaintext: Option<String>,
	pub key: Option<C::Key>,
	pub start: Option<C::Key>,
	pub end: Option<C::Key>,
}

impl<C: Cipher> Arguments<C> {
	fn new() -> Arguments<C> {
		Arguments {
			language_model: None,
			dictionary: None,
			ciphertext: None,
			plaintext: None,
			key: None,
			start: None,
			end: None,
		}
	}
}

pub fn parse_available<'a, C: Cipher>(matches: &ArgMatches<'a>) -> Arguments<C> {
	let mut a = Arguments::<C>::new();

	a.language_model = match matches.value_of("language") {
		None => None,
		Some(filename) => {
			Some(language_model(filename))
		}
	};

	a.dictionary = match matches.value_of("dict-file") {
		None => None,
		Some(filename) => {
			//TODO string conversion shouldn't be necessary, but lifetimes get
			//a bit complicated with &str
			//https://users.rust-lang.org/t/box-lifetime-problem/9030
			Some(dictionary::<C>(String::from(filename)))
		}
	};

	a.ciphertext = match matches.value_of("ciphertext") {
		None => None,
		Some(filename) => {
			Some(text(filename))
		}
	};

	a.plaintext = match matches.value_of("plaintext") {
		None => None,
		Some(filename) => {
			Some(text(filename))
		}
	};

	a.key = match matches.value_of("key") {
		None => None,
		Some(key_str) => {
			Some(key::<C>(key_str))
		}
	};

	a.start = match matches.value_of("start") {
		None => None,
		Some(key_str) => {
			Some(key::<C>(key_str))
		}
	};

	a.end = match matches.value_of("end") {
		None => None,
		Some(key_str) => {
			Some(key::<C>(key_str))
		}
	};

	a
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

pub fn dictionary<C>(filename: String) -> Box<dyn Iterator<Item = C::Key>> where
	C: Cipher,
{
	let file = match File::open(&filename) {
		Err(why) => {
			eprintln!("{}: {}", filename, why);
			process::exit(1);
		}
		Ok(file) => file,
	};

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
					eprintln!("{}:{}: failed to parse \"{}\"", filename, line_num, line);
					process::exit(1);
				}
				Ok(key) => key
			}
			});

	Box::new(dict)
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

