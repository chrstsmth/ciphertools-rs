extern crate serde;
extern crate serde_json;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

mod cipher;
mod pallet;

use cipher::vigenere::*;
use cipher::Cipher;

fn main() {
	let v = Vigenere::new();

	let matches = App::new("Cipher Tools")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(SubCommand::with_name("vigenere")
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(SubCommand::with_name("encipher")
				.about("Encipher plaintext")
				.arg(Arg::with_name("ciphertext")
					.short("c")
					.value_name("CIPHERTEXT")
					.required(true))
				.arg(Arg::with_name("key")
					.short("k")
					.value_name("KEY")
					.required(true)))
			.subcommand(SubCommand::with_name("decipher")
				.about("Decipher ciphertext")
				.arg(Arg::with_name("plaintext")
					.short("p")
					.value_name("PLAINTEXT")
					.required(true))
				.arg(Arg::with_name("key")
					.short("k")
					.value_name("KEY")
					.required(true))))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("vigenere") {
		if let Some(matches) = matches.subcommand_matches("encipher") {
			let ciphertext = matches.value_of("ciphertext").unwrap();
			let key = matches.value_of("key").unwrap();
			println!("{:}", v.encipher(String::from(ciphertext), String::from(key)));
		} else if let Some(matches) = matches.subcommand_matches("deciphier") {
			let plaintext = matches.value_of("plaintext").unwrap();
			let key = matches.value_of("key").unwrap();
			println!("{:}", v.encipher(String::from(plaintext), String::from(key)));
		}
	}
}
