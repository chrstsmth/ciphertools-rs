use std::fs::File;
use std::process;

use cipher_lib::cipher::*;
use cipher_lib::language_model::*;
use std::io::prelude::*;
use std::io::*;
use std::str::FromStr;
use std::vec::*;

pub struct ParsedArg<'a, T> {
	pub value: T,
	pub args: Vec<&'a str>,
	pub i: usize,
}

pub fn dictionary_option<C: Cipher>(
	matches: &clap::ArgMatches,
) -> Option<Box<dyn Iterator<Item = C::Key>>> {
	let dict_file = matches.value_of("dictionary")?;
	Some(dictionary_file::<C>(&dict_file))
}

pub fn language_model_option(matches: &clap::ArgMatches) -> Option<LanguageModel> {
	Some(language_model(matches.value_of("language")?))
}

pub fn ciphertext_option(matches: &clap::ArgMatches) -> Option<String> {
	Some(text(matches.value_of("ciphertext")?))
}

pub fn plaintext_option(matches: &clap::ArgMatches) -> Option<String> {
	Some(text(matches.value_of("plaintext")?))
}

pub fn text_option(matches: &clap::ArgMatches) -> Option<String> {
	Some(text(matches.value_of("text")?))
}

pub fn key_option<C: Cipher>(matches: &clap::ArgMatches) -> Option<C::Key> {
	Some(key::<C>(matches.value_of("key")?))
}

pub fn language_model_options<'a>(
	matches: &'a clap::ArgMatches,
) -> Option<Vec<ParsedArg<'a, LanguageModel>>> {
	let files = matches.values_of("language")?;
	let indices = matches.indices_of("language")?;
	let mut language_models = Vec::new();

	for (i, f) in indices.zip(files) {
		language_models.push(ParsedArg {
			value: language_model(f),
			args: vec![f],
			i,
		});
	}
	Some(language_models)
}

pub fn text_options<'a>(matches: &'a clap::ArgMatches) -> Option<Vec<ParsedArg<'a, String>>> {
	let files = matches.values_of("text")?;
	let indices = matches.indices_of("text")?;
	let mut texts = Vec::new();

	for (i, f) in indices.zip(files) {
		texts.push(ParsedArg {
			value: text(f),
			args: vec![f],
			i,
		});
	}
	Some(texts)
}

pub fn dictionary_file<C: Cipher>(filename: &str) -> Box<dyn Iterator<Item = C::Key>> {
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
		.map(|x| {
			x.unwrap_or_else(|e| {
				eprintln!("{}", e.to_string());
				process::exit(1);
			})
		})
		.enumerate()
		.map(move |x| {
			let (num, line) = x;
			let line_num = num + 1;
			match C::Key::from_str(line.as_str()) {
				Err(_) => {
					eprintln!(
						"{}:{}: failed to parse \"{}\"",
						filename_clone, line_num, line
					);
					process::exit(1);
				}
				Ok(key) => key,
			}
		});

	Box::new(dict)
}

pub fn language_model(filename: &str) -> LanguageModel {
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

pub fn key<'a, C: Cipher>(key_str: &str) -> C::Key {
	match C::Key::from_str(key_str) {
		Ok(key) => key,
		_ => {
			println!("{}: Parse key failed", key_str);
			process::exit(1);
		}
	}
}

fn text<'a>(filename: &str) -> String {
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
		}
		Ok(_) => (),
	}

	text
}
