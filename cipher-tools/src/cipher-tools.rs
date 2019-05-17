extern crate ctrlc;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate cipher_lib;

use clap::{Arg, App, SubCommand, AppSettings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::error::Error;
use std::process;

mod try_from_err;

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;
use cipher_lib::key::*;

macro_rules! key_arg {
	() => (
		Arg::with_name("key")
			.short("k")
			.value_name("KEY")
			.required(true)
		)
}

macro_rules! decipher_subcommand {
	() => (
		SubCommand::with_name("decipher")
		.about("Decipher ciphertext")
		.arg(Arg::with_name("ciphertext")
			.short("c")
			.value_name("CIPHERTEXT")
			.required(true))
		.arg(key_arg!())
	)
}

macro_rules! encipher_subcommand {
	() => (
		SubCommand::with_name("encipher")
		.about("Encipher plaintext")
		.arg(Arg::with_name("plaintext")
			.short("p")
			.value_name("PLAINTEXT")
			.required(true))
		.arg(key_arg!())
	)
}

macro_rules! dictionary_attack_subcommand {
	() => (
		SubCommand::with_name("dictionary")
					.about("Dictionary attack")
					.arg(Arg::with_name("ciphertext")
						.short("c")
						.value_name("CIPHERTEXT")
						.required(true))
					.arg(Arg::with_name("dictionary")
						.short("d")
						.value_name("DICTIONARY")
						.required(true))
					.arg(Arg::with_name("language")
						.short("l")
						.value_name("LANGUAGE")
						.required(true))
	)
}

macro_rules! brute_force_subcommand {
	() => (
		SubCommand::with_name("brute")
					.about("Brute force")
					.arg(Arg::with_name("ciphertext")
						.short("c")
						.value_name("CIPHERTEXT")
						.required(true))
					.arg(Arg::with_name("language")
						.short("l")
						.value_name("LANGUAGE")
						.required(true))
	)
}

macro_rules! encipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("encipher") {
			let plaintext = String::from(matches.value_of("plaintext").unwrap());
			let key = <$Cipher as Cipher>::Key::try_from(String::from(matches.value_of("key").unwrap()));

			match key {
				Ok(key) => println!("{:}", $Cipher::encipher(&plaintext, &key)),
				_ => println!("Parse key failed"),
			}
		}
	)
}

macro_rules! decipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("decipher") {
			let ciphertext = String::from(matches.value_of("ciphertext").unwrap());
			let key = <$Cipher as Cipher>::Key::try_from(String::from(matches.value_of("key").unwrap()));

			match key {
				Ok(key) => println!("{:}", $Cipher::decipher(&ciphertext, &key)),
				_ => println!("Parse key failed"),
			}
		}
	)
}

macro_rules! dictionary_attack {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("dictionary") {
			let ciphertext = String::from(matches.value_of("ciphertext").unwrap());
			let dictionary = String::from(matches.value_of("dictionary").unwrap());
			let language = String::from(matches.value_of("language").unwrap());

			let dictionary_file = match File::open(&dictionary) {
				Err(why) => {
					eprintln!("{}: {}", dictionary, why);
					process::exit(1);
				}
				Ok(file) => file,
			};

			let dict = BufReader::new(dictionary_file)
				.lines()
				.map(|x| x.unwrap_or_else(|e|
					{
						eprintln!("{}", e.description());
						process::exit(1);
					})
					)
				.enumerate()
				.map(|x| {
					let (num, line) = x;
					let line_num = num + 1;
					match <$Cipher as Cipher>::Key::try_from(line.clone()) {
						Err(why) => {
							eprintln!("Error in {}:{}\n{}: {}", dictionary, line_num, line, why.description());
							process::exit(1);
						}
						Ok(key) => key
					}
					});

			let language_file = match File::open(&language) {
				Err(why) => {
					eprintln!("{}: {}", language, why);
					process::exit(1);
				}
				Ok(file) => file,
			};

			let language_reader = BufReader::new(language_file);

			let lang = match serde_json::from_reader(language_reader) {
				Err(why) => {
					eprintln!("{}: {}", language, why);
					process::exit(1);
				}
				Ok(language) => language,
			};

			let candidates = $Cipher::dictionary_attack(&ciphertext, dict, 10, lang, $exit.clone());

			for candidate in candidates {
				println!("{}", candidate);
			}
		}
	)
}

macro_rules! brute_force {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("brute") {
			let ciphertext = String::from(matches.value_of("ciphertext").unwrap());
			let language = String::from(matches.value_of("language").unwrap());

			let language_file = match File::open(&language) {
				Err(why) => {
					eprintln!("{}: {}", language, why);
					process::exit(1);
				}
				Ok(file) => file,
			};

			let language_reader = BufReader::new(language_file);

			let lang = match serde_json::from_reader(language_reader) {
				Err(why) => {
					eprintln!("{}: {}", language, why);
					process::exit(1);
				}
				Ok(language) => language,
			};

			type Key = <<$Cipher as Cipher>::Key as IntoBruteForceIterator>::BruteForceIter;
			let candidates = <$Cipher as BruteForce<Key>>::brute_force(&ciphertext, 10, lang, $exit.clone());

			for candidate in candidates {
				println!("{}", candidate);
			}
		}
	)
}


fn main() {
	let exit = Arc::new(AtomicBool::new(false));
	let ctrlc_exit = exit.clone();
	ctrlc::set_handler(move ||  {
		ctrlc_exit.store(true, Ordering::SeqCst);
	}).expect("Error setting SIGINT trap");


	let vigenere = "vigenere";
	let caesar = "caesar";

	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(SubCommand::with_name(vigenere)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand!())
			.subcommand(decipher_subcommand!())
			.subcommand(dictionary_attack_subcommand!())
			.subcommand(brute_force_subcommand!()))

		.subcommand(SubCommand::with_name(caesar)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand!())
			.subcommand(decipher_subcommand!())
			.subcommand(dictionary_attack_subcommand!())
			.subcommand(brute_force_subcommand!()))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(vigenere) {
		encipher!(matches, Vigenere);
		decipher!(matches, Vigenere);
		dictionary_attack!(matches, Vigenere, exit);
		brute_force!(matches, Vigenere, exit);
	} else if let Some(matches) = matches.subcommand_matches(caesar) {
		encipher!(matches, Caesar);
		decipher!(matches, Caesar);
		dictionary_attack!(matches, Caesar, exit);
		brute_force!(matches, Caesar, exit);
	}
}
