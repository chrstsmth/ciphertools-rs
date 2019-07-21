extern crate ctrlc;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate cipher_lib;
#[macro_use]
extern crate lazy_static;

use clap::{App, AppSettings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::prelude::*;
use std::io::*;
use std::io;
use std::process;
use std::str::FromStr;
use std::convert::TryFrom;

mod try_from_err;
mod cli;
mod parse;

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;
use cipher_lib::key::*;
use cipher_lib::candidate::*;
use cipher_lib::pallet::lang::*;
use cipher_lib::score::*;

lazy_static! {
	static ref HAS_STDIN: bool = has_stdin();
}

fn has_stdin() -> bool {
	BufReader::new(io::stdin()).lines().peekable().next().is_some()
}

macro_rules! encipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("encipher") {
			let plaintext = parse::plaintext(matches).unwrap();
			let key = parse::key::<$Cipher>(matches).unwrap();

			println!("{:}", $Cipher::encipher(&plaintext, &key));
		}
	)
}

macro_rules! decipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("decipher") {
			let ciphertext = parse::ciphertext(matches).unwrap();
			let key = parse::key::<$Cipher>(matches).unwrap();

			println!("{:}", $Cipher::decipher(&ciphertext, &key));
		}
	)
}

macro_rules! dictionary_attack {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("dictionary") {
			let ciphertext = parse::ciphertext(matches).unwrap();
			let language  = parse::language_model(matches).unwrap();
			let dictionary = parse::dictionary::<$Cipher>(matches).unwrap();

			let mut candidates = Candidates::<$Cipher>::with_capacity(10); let insert_candidate = |c: &Candidate<$Cipher>| {
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

			let ciphertext = parse::ciphertext(matches).unwrap();
			let language  = parse::language_model(matches).unwrap();

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
			} else if let Some(end) = end {
				<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force_to(&ciphertext, end, score, insert_candidate, exit_early);
			} else {
				<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force(&ciphertext, score, insert_candidate, exit_early);
			};
		}
	)
}

macro_rules! hill_climb {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("hill") {
			let ciphertext = parse::ciphertext(matches).unwrap();
			let language = parse::language_model(matches).unwrap();
			let dictionary = parse::dictionary::<$Cipher>(matches).unwrap();

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

fn main() {
	//let stdin = BufReader::new(io::stdin()).lines();

	let exit = Arc::new(AtomicBool::new(false));

	let ctrlc_exit = exit.clone();
	ctrlc::set_handler(move ||  {
		ctrlc_exit.store(true, Ordering::SeqCst);
	}).expect("Error setting SIGINT trap");

	//ensures arguments are valid
	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(cli::vigenere_subcommand())
		.subcommand(cli::caesar_subcommand())
		.get_matches();

	// TODO Parse arguments given

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
