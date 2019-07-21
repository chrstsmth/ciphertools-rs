use clap::ArgMatches;
use std::fs::File;
use std::process;

use cipher_lib::language_model::*;
use cipher_lib::cipher::*;
use std::io::prelude::*;
use std::io::*;
use std::error::Error;
use std::str::FromStr;

//TODO pass in filename?
pub fn language_model<'a>(matches: &ArgMatches<'a>) -> Option<LanguageModel>
{
	let filename = String::from(matches.value_of("language")?);

	let file = match File::open(&filename) {
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

	Some(language)
}

pub fn dictionary<'a, C>(matches: &ArgMatches<'a>) -> Option<impl Iterator<Item = C::Key>> where
	C: Cipher,
{
	let filename = String::from(matches.value_of("dictionary")?);

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

	Some(dict)
}

pub fn key<'a, C: Cipher>(matches: &ArgMatches<'a>) -> Option<C::Key>
{
	let key_str = matches.value_of("key")?;
	let key = C::Key::from_str(key_str);

	match key {
		Ok(key) => Some(key),
		_ => {
			//TODO retur as error
			println!("{}: Parse key failed", key_str);
			process::exit(1);
		}
	}
}

pub fn ciphertext<'a>(matches: &ArgMatches<'a>) -> Option<String>
{
	text(matches, "ciphertext")
}

pub fn plaintext<'a>(matches: &ArgMatches<'a>) -> Option<String>
{
	text(matches, "plaintext")
}

fn text<'a>(matches: &ArgMatches<'a>, text: &str) -> Option<String>
{
	let filename = String::from(matches.value_of(text)?);

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
		},
		Ok(_) => (),
	}

	Some(text)
}

