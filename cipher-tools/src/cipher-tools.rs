extern crate ctrlc;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate cipher_lib;

use clap::{Arg, ArgGroup, App, SubCommand, AppSettings, ArgMatches};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::io;
use std::error::Error;
use std::process;
use std::str::FromStr;
use std::convert::TryFrom;

mod try_from_err;

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;
use cipher_lib::key::*;
use cipher_lib::candidate::*;
use cipher_lib::language_model::*;
use cipher_lib::pallet::lang::*;
use cipher_lib::score::*;



mod arg {
	use super::*;

	pub fn key<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("key")
			.short("k")
			.value_name("KEY")
			.required(true)
	}

	pub fn ciphertext<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("ciphertext")
			.short("c")
			.value_name("CIPHERTEXT")
			.required(true)
	}

	pub fn plaintext<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("plaintext")
			.short("p")
			.value_name("PLAINTEXT")
			.required(true)
	}

	pub fn language_model<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("language")
			.short("l")
			.value_name("LANGUAGE")
			.required(true)
	}

	pub fn dictionary<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dictionary")
			.short("d")
			.value_name("DICTIONARY")
			.required(true)
	}
}

mod subcommand {
	use super::*;

	pub fn dictionary_attack<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("dictionary")
			.about("Dictionary attack")
			.arg(arg::ciphertext())
			.arg(arg::language_model())
			.arg(arg::dictionary())
	}

	pub fn decipher<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("decipher")
			.about("Decipher ciphertext")
			.arg(arg::ciphertext())
			.arg(arg::key())
	}

	pub fn encipher<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("encipher")
			.about("Encipher plaintext")
			.arg(arg::plaintext())
			.arg(arg::key())
	}

	pub fn brute_force<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("brute") .about("Brute force") .arg(arg::ciphertext()) .arg(arg::language_model())
			.arg(Arg::with_name("start")
				.short("s")
				.value_name("START-KEY")
				.required(false))
			.arg(Arg::with_name("end")
				.short("e")
				.value_name("END-KEY")
				.required(false))
	}

	pub fn hill_climb<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("hill")
			.about("Hill climb")
			.arg(arg::ciphertext())
			.arg(arg::language_model())
			.arg(arg::dictionary())
	}
}

mod parse {
	use super::*;

	pub fn language_model_parse<'a>(matches: &ArgMatches<'a>) -> LanguageModel
	{
		let filename = String::from(matches.value_of("language").unwrap());

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

		language
	}

	pub fn dictionary_parse<'a, C>(matches: &ArgMatches<'a>) -> impl Iterator<Item = C::Key> where
		C: Cipher,
	{
		let filename = String::from(matches.value_of("dictionary").unwrap());

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

		dict
	}

	pub fn key_parse<'a, C: Cipher>(matches: &ArgMatches<'a>) -> C::Key
	{
		let key_str = matches.value_of("key").unwrap();
		let key = C::Key::from_str(key_str);

		match key {
			Ok(key) => key,
			_ => {
				println!("{}: Parse key failed", key_str);
				process::exit(1);
			}
		}
	}

	pub fn ciphertext_parse<'a>(matches: &ArgMatches<'a>) -> String
	{
		text_parse(matches, "ciphertext")
	}

	pub fn plaintext_parse<'a>(matches: &ArgMatches<'a>) -> String
	{
		text_parse(matches, "plaintext")
	}

	fn text_parse<'a>(matches: &ArgMatches<'a>, text: &str) -> String
	{
		let filename = String::from(matches.value_of(text).unwrap());

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

		text
	}
}

macro_rules! encipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("encipher") {
			let plaintext = parse::plaintext_parse(matches);
			let key = parse::key_parse::<$Cipher>(matches);

			println!("{:}", $Cipher::encipher(&plaintext, &key));
		}
	)
}

macro_rules! decipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("decipher") {
			let ciphertext = parse::ciphertext_parse(matches);
			let key = parse::key_parse::<$Cipher>(matches);

			println!("{:}", $Cipher::decipher(&ciphertext, &key));
		}
	)
}

macro_rules! dictionary_attack {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("dictionary") {
			let ciphertext = parse::ciphertext_parse(matches);
			let language  = parse::language_model_parse(matches);
			let dictionary = parse::dictionary_parse::<$Cipher>(matches);

			let mut candidates = Candidates::<$Cipher>::with_capacity(10);
			let insert_candidate = |c: &Candidate<$Cipher>| {
				if candidates.insert_candidate(c) {
					print!("{}[2J", 27 as char);
					println!("{}", candidates);
				}
			};

			let score = |chars: std::str::Chars| {
				let alph = chars
					.map(|x| Lang::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap());

					let tr = language.traverse();
					score(tr, alph.clone())
			};

			let exit_early = || {
				$exit.load(Ordering::SeqCst)
			};

			$Cipher::dictionary_attack(&ciphertext, dictionary, score, insert_candidate, exit_early);
		}
	)
}

macro_rules! brute_force {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("brute") {
			type BruteForceIter = <<$Cipher as Cipher>::Key as IntoBruteForceIterator>::BruteForceIter;
			type Key = <$Cipher as Cipher>::Key;

			let ciphertext = parse::ciphertext_parse(matches);
			let language  = parse::language_model_parse(matches);

			let start = match matches.value_of("start") {
				Some(key_str) => {
					match Key::from_str(key_str) {
						Ok(key) => Some(key),
						Err(why) => {
							eprintln!("{}: {}", key_str, why);
							process::exit(1);
						}
					}
				}
				None => None
			};

			let end = match matches.value_of("end") {
				Some(key_str) => {
					match Key::from_str(key_str) {
						Ok(key) => Some(key),
						Err(why) => {
							eprintln!("{}: {}", key_str, why);
							process::exit(1);
						}
					}
				}
				None => None
			};

			let mut candidates = Candidates::<$Cipher>::with_capacity(10);
			let insert_candidate = |c: &Candidate<$Cipher>| {
				if candidates.insert_candidate(c) {
					print!("{}[2J", 27 as char);
					println!("{}", candidates);
				}
			};

			let exit_early = || {
				$exit.load(Ordering::SeqCst)
			};

			let score = |chars: std::str::Chars| {
				let alph = chars
					.map(|x| Lang::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap());

					let tr = language.traverse();
					score(tr, alph)
			};

			if let Some(start) = start {
				if let Some(end) = end {
					<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force_between(&ciphertext, start, end, score, insert_candidate, exit_early);
				} else {
					<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force_from(&ciphertext, start, score, insert_candidate, exit_early);
				}
			} else {
				<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force(&ciphertext, score, insert_candidate, exit_early);
			};
		}
	)
}

macro_rules! hill_climb {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("hill") {
			let ciphertext = parse::ciphertext_parse(matches);
			let language = parse::language_model_parse(matches);
			let dictionary = parse::dictionary_parse::<$Cipher>(matches);

			let mut candidates = Candidates::<$Cipher>::with_capacity(10);
			let insert_candidate = |c: &Candidate<$Cipher>| {
				if candidates.insert_candidate(c) {
					print!("{}[2J", 27 as char);
					println!("{}", candidates);
				}
			};

			let exit_early = || {
				$exit.load(Ordering::SeqCst)
			};

			let score = |chars: std::str::Chars| {
				let alph = chars
					.map(|x| Lang::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap());

					let tr = language.traverse();
					score(tr, alph)
			};

			$Cipher::hill_climb(&ciphertext, dictionary, score, insert_candidate, exit_early);
		}
	)
}

fn vigenere_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name(Vigenere::NAME)
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(subcommand::encipher())
		.subcommand(subcommand::decipher())
		.subcommand(subcommand::dictionary_attack())
		.subcommand(subcommand::brute_force())
		.subcommand(subcommand::hill_climb())
}

fn caesar_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name(Caesar::NAME)
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(subcommand::encipher())
		.subcommand(subcommand::decipher())
		.subcommand(subcommand::brute_force())
}

fn main() {
	let exit = Arc::new(AtomicBool::new(false));
	let ctrlc_exit = exit.clone();
	ctrlc::set_handler(move ||  {
		ctrlc_exit.store(true, Ordering::SeqCst);
	}).expect("Error setting SIGINT trap");

	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(vigenere_subcommand())
		.subcommand(caesar_subcommand())
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(Vigenere::NAME) {
		encipher!(matches, Vigenere);
		decipher!(matches, Vigenere);
		dictionary_attack!(matches, Vigenere, exit);
		brute_force!(matches, Vigenere, exit);
		hill_climb!(matches, Vigenere, exit);
	} else if let Some(matches) = matches.subcommand_matches(Caesar::NAME) {
		encipher!(matches, Caesar);
		decipher!(matches, Caesar);
		dictionary_attack!(matches, Caesar, exit);
		brute_force!(matches, Caesar, exit);
	}
}
