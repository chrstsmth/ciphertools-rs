extern crate clap;
extern crate cipher_lib;

mod cli;
mod commands;
mod parse;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use cipher_lib::key::vigenere::*;
use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;
use cli::*;
use commands::Random;

use clap::{App, AppSettings};

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

	let matches = App::new("Dictionary")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(Vigenere::command())
		.subcommand(Caesar::command())
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(Vigenere::NAME) {
		if let Some(matches) = matches.subcommand_matches("range") {
			commands::range::<<Vigenere as Cipher>::Key, _>(matches, exit_early);
		} if let Some(matches) = matches.subcommand_matches("random") {
			VigenereKey::random(matches, exit_early);
		}
	} else if let Some(matches) = matches.subcommand_matches(Caesar::NAME) {
		if let Some(matches) = matches.subcommand_matches("range") {
			commands::range::<<Caesar as Cipher>::Key, _>(matches, exit_early)
		}
	}
}
