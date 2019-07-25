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
#[macro_use]
mod commands;
mod parse;

use cli::Cli;
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

fn main() {
	//let stdin = BufReader::new(io::stdin()).lines();

	let exit = Arc::new(AtomicBool::new(false));

	let ctrlc_exit = exit.clone();
	ctrlc::set_handler(move ||  {
		ctrlc_exit.store(true, Ordering::SeqCst);
	}).expect("Error setting SIGINT trap");

	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(Vigenere::command())
		.subcommand(Caesar::command())
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
