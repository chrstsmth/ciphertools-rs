extern crate ctrlc;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate cipher_lib;

use clap::{App, AppSettings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod try_from_err;
mod cli;
#[macro_use]
mod commands;
mod parse;

use cli::*;
use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;

fn main() {
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

	if let Some(matches) = matches.subcommand_matches(Vigenere::NAME) {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			commands::encipher::<Vigenere>(&matches);
		} else if let Some(matches) = matches.subcommand_matches("decipher") {
			commands::decipher::<Vigenere>(&matches);
		} else if let Some(matches) = matches.subcommand_matches("dictionary") {
			commands::dictionary_attack::<Vigenere,_>(&matches, exit_early);
		}
	} else if let Some(matches) = matches.subcommand_matches(Caesar::NAME) {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			commands::encipher::<Caesar>(&matches);
		} else if let Some(matches) = matches.subcommand_matches("decipher") {
			commands::decipher::<Caesar>(&matches);
		}
	}
}
