use clap::{App, AppSettings, SubCommand};

use common::cli::*;

use cipher_lib::cipher::caesar::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::*;

fn dictionary_attack_subcommand<'a, 'b>() -> App<'a, 'b>
{
	SubCommand::with_name("dictionary")
		.about("Dictionary attack")
		.arg(ciphertext_arg().required(true))
		.arg(language_model_arg().required(true))
		.arg(dictionary_arg().required(true))
}

fn hill_climb_subcommand<'a, 'b>() -> App<'a, 'b>
{
	SubCommand::with_name("hillclimb")
		.about("Hill climb")
		.arg(ciphertext_arg().required(true))
		.arg(language_model_arg().required(true))
		.arg(dictionary_arg().required(true))
}

fn decipher_subcommand<'a, 'b>() -> App<'a, 'b>
{
	SubCommand::with_name("decipher")
		.about("Decipher ciphertext")
		.arg(ciphertext_arg().required(true))
		.arg(key_arg().required(true))
}

fn encipher_subcommand<'a, 'b>() -> App<'a, 'b>
{
	SubCommand::with_name("encipher")
		.about("Encipher plaintext")
		.arg(plaintext_arg().required(true))
		.arg(key_arg().required(true))
}

pub trait Cli {
	fn command<'a, 'b>() -> App<'a, 'b>;
}

impl Cli for Caesar {
	fn command<'a, 'b>() -> App<'a, 'b> {
		SubCommand::with_name(Caesar::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand())
			.subcommand(decipher_subcommand())
	}
}

impl Cli for Vigenere {
	fn command<'a, 'b>() -> App<'a, 'b> {
		SubCommand::with_name(Vigenere::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand())
			.subcommand(decipher_subcommand())
			.subcommand(dictionary_attack_subcommand())
			.subcommand(hill_climb_subcommand())
	}
}
