use clap::{App, AppSettings, SubCommand};

use common::cli::*;

use cipher_lib::cipher::caesar::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::*;

fn dictionary_attack_subcommand<'a, 'b, C>() -> App<'a, 'b>
where
	C: DictionaryAttack,
{
	SubCommand::with_name("dictionary")
		.about("Dictionary attack")
		.arg(ciphertext_arg().required(true))
		.arg(language_model_arg().required(true))
		.arg(dictionary_arg().required(true))
}

fn hill_climb_subcommand<'a, 'b, C>() -> App<'a, 'b>
where
	C: HillClimb,
{
	SubCommand::with_name("hillclimb")
		.about("Hill climb")
		.arg(ciphertext_arg().required(true))
		.arg(language_model_arg().required(true))
		.arg(dictionary_arg().required(true))
}

fn decipher_subcommand<'a, 'b, C>() -> App<'a, 'b>
where
	C: Cipher,
{
	SubCommand::with_name("decipher")
		.about("Decipher ciphertext")
		.arg(ciphertext_arg().required(true))
		.arg(key_arg().required(true))
}

fn encipher_subcommand<'a, 'b, C>() -> App<'a, 'b>
where
	C: Cipher,
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
			.subcommand(encipher_subcommand::<Self>())
			.subcommand(decipher_subcommand::<Self>())
	}
}

impl Cli for Vigenere {
	fn command<'a, 'b>() -> App<'a, 'b> {
		SubCommand::with_name(Vigenere::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand::<Self>())
			.subcommand(decipher_subcommand::<Self>())
			.subcommand(dictionary_attack_subcommand::<Self>())
			.subcommand(hill_climb_subcommand::<Self>())
	}
}
