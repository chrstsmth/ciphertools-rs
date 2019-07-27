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
use std::convert::TryFrom;
use std::str::Chars;

mod try_from_err;
mod cli;
#[macro_use]
mod commands;
mod parse;

use parse::*;
use cli::*;
use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;
use cipher_lib::candidate::*;
use cipher_lib::pallet::lang::*;
use cipher_lib::score::*;
use cipher_lib::language_model::*;

lazy_static! {
	static ref HAS_STDIN: bool = has_stdin();
}

fn has_stdin() -> bool {
	BufReader::new(io::stdin()).lines().peekable().next().is_some()
}

fn insert_candidates<C: Cipher>() -> impl FnMut(&Candidate<C>) {
	let mut candidates = Candidates::<C>::with_capacity(10);
	move |c: &Candidate<C>| {
		if candidates.insert_candidate(c) {
			print!("{}[2J", 27 as char);
			println!("{}", candidates);
		}
	}
}

fn score_candidate(language_model: LanguageModel) -> impl Fn(Chars) -> u32 {
	move |chars: std::str::Chars| {
		let alph = chars
			.map(|x| Lang::try_from(x))
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap());

			let tr = language_model.traverse();
			score(tr, alph)
	}
}

fn main() {
	//let stdin = BufReader::new(io::stdin()).lines();

	let exit = Arc::new(AtomicBool::new(false));
	let exit_early = || {
		exit.load(Ordering::SeqCst)
	};

	{
		let ctrlc_exit = exit.clone();
		ctrlc::set_handler(move ||  {
			ctrlc_exit.store(true, Ordering::SeqCst);
		}).expect("Error setting SIGINT trap");
	}

	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(Vigenere::command())
		.subcommand(Caesar::command())
		.get_matches();

	// TODO Parse arguments given


	if let Some(matches) = matches.subcommand_matches(Vigenere::NAME) {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			let args = parse_available::<Vigenere>(&matches);
			println!("{}", Vigenere::encipher(&args.plaintext.unwrap(), &args.key.unwrap()));
		} else if let Some(matches) = matches.subcommand_matches("decipher") {
			let args = parse_available::<Vigenere>(&matches);
			println!("{}", Vigenere::decipher(&args.ciphertext.unwrap(), &args.key.unwrap()));
		} else if let Some(matches) = matches.subcommand_matches("dictionary") {
			let args = parse_available::<Vigenere>(&matches);
			Vigenere::dictionary_attack(
				&args.ciphertext.unwrap(),
				args.dictionary.unwrap(),
				score_candidate(args.language_model.unwrap()),
				insert_candidates(),
				exit_early);
		}
	} else if let Some(matches) = matches.subcommand_matches(Caesar::NAME) {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			let args = parse_available::<Caesar>(&matches);
			println!("{}", Caesar::encipher(&args.plaintext.unwrap(), &args.key.unwrap()));
		} else if let Some(matches) = matches.subcommand_matches("decipher") {
			let args = parse_available::<Caesar>(&matches);
			println!("{}", Caesar::decipher(&args.ciphertext.unwrap(), &args.key.unwrap()));
		}
	}
}
