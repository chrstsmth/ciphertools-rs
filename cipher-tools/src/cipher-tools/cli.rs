use clap::{Arg, App, SubCommand, AppSettings};

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;

pub trait Cli {
	fn key_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("key")
			.short("k")
			.value_name("KEY")
			.required(true)
	}

	fn ciphertext_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("ciphertext")
			.short("c")
			.value_name("CIPHERTEXT")
			.required(true)
	}

	fn plaintext_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("plaintext")
			.short("p")
			.value_name("PLAINTEXT")
			.required(true)
	}
	fn language_model_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("language")
			.short("l")
			.value_name("LANGUAGE")
			.required(true)
	}

	fn dict_file_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("dict_file")
			.short("dict_file")
			.value_name("FILE")
			.required(true)
	}

	fn dict_random_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("dict_random")
			.long("dict_random")
			.required(true)
	}

	fn dict_range_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("dict_range")
			.long("dict_range")
			.value_name("(START, END)")
			.required(true)
	}

	fn dict_brute_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("dict_brute")
			.long("dict_brute")
			.required(true)
	}

	fn dict_stdin_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("dict_stdin")
			.long("dict_stdin")
			.required(true)
	}

	fn start_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("start")
			.short("s")
			.value_name("START-KEY")
			.required(true)
	}

	fn end_arg<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("end")
			.short("e")
			.value_name("END-KEY")
			.required(true)
	}

	fn dictionary_attack_subcommand<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name("dictionary")
			.about("Dictionary attack")
			.arg(Self::ciphertext_arg())
			.arg(Self::language_model_arg())
			.arg(Self::dict_file_arg())
	}

	fn decipher_subcommand<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name("decipher")
			.about("Decipher ciphertext")
			.arg(Self::ciphertext_arg())
			.arg(Self::key_arg())
	}

	fn encipher_subcommand<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name("encipher")
			.about("Encipher plaintext")
			.arg(Self::plaintext_arg())
			.arg(Self::key_arg())
	}

	fn brute_force_subcommand<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name("brute")
			.about("Brute force")
			.arg(Self::ciphertext_arg())
			.arg(Self::language_model_arg())
			.arg(Self::start_arg().required(false))
			.arg(Self::end_arg().required(false))
	}

	fn hill_climb_subcommand<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name("hill")
			.about("Hill climb")
			.arg(Self::ciphertext_arg())
			.arg(Self::language_model_arg())
			.arg(Self::dict_file_arg())
	}

	fn command<'a,'b>() -> App<'a,'b>;
}

impl Cli for Caesar {
	fn command<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name(Caesar::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(Self::encipher_subcommand())
			.subcommand(Self::decipher_subcommand())
			.subcommand(Self::brute_force_subcommand())
	}
}

impl Cli for Vigenere {
	fn command<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name(Vigenere::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(Self::encipher_subcommand())
			.subcommand(Self::decipher_subcommand())
			.subcommand(Self::dictionary_attack_subcommand())
			.subcommand(Self::brute_force_subcommand())
			.subcommand(Self::hill_climb_subcommand())
	}
}

