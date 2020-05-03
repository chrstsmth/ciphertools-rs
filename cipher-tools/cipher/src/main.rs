extern crate cipher_lib;
extern crate clap;
extern crate common;
extern crate ctrlc;
extern crate itertools;
extern crate serde;
extern crate serde_json;

use clap::{App, AppSettings};

mod cli;
#[macro_use]
mod command;

use cipher_lib::cipher::caesar::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::*;
use cli::*;
use command::*;

fn main() {
	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(Vigenere::command())
		.subcommand(Caesar::command())
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(Vigenere::NAME) {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			encipher_command::<Vigenere>(&matches, &());
		} else if let Some(matches) = matches.subcommand_matches("decipher") {
			decipher_command::<Vigenere>(&matches, &());
		} else if let Some(matches) = matches.subcommand_matches("dictionary") {
			dictionary_attack_command::<Vigenere>(&matches, ());
		} else if let Some(matches) = matches.subcommand_matches("hillclimb") {
			hillclimb_command::<Vigenere>(&matches, &());
		}
	} else if let Some(matches) = matches.subcommand_matches(Caesar::NAME) {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			encipher_command::<Caesar>(&matches, &());
		} else if let Some(matches) = matches.subcommand_matches("decipher") {
			decipher_command::<Caesar>(&matches, &());
		}
	}
}
