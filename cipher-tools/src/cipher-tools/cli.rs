use clap::{Arg, ArgGroup, App, SubCommand, AppSettings};

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;

fn key_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("key")
		.short("k")
		.value_name("KEY")
}

fn ciphertext_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("ciphertext")
		.short("c")
		.value_name("CIPHERTEXT")
}

fn plaintext_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("plaintext")
		.short("p")
		.value_name("PLAINTEXT")
}
fn language_model_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("language")
		.short("l")
		.value_name("LANGUAGE")
}

fn dict_file_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("dict_file")
		.short("dict_file")
		.value_name("FILE")
}

fn dict_random_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("dict_random")
		.long("dict_random")
}

fn dict_range_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("dict_range")
		.long("dict_range")
		.value_name("(START, END)")
}

fn dict_brute_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("dict_brute")
		.long("dict_brute")
}

fn dict_stdin_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("dict_stdin")
		.long("dict_stdin")
}

fn dictionary_attack_subcommand<'a,'b, C>() -> App<'a,'b>
where
	C: DictionaryAttack + DictionaryArgs
{
	SubCommand::with_name("dictionary")
		.about("Dictionary attack")
		.arg(ciphertext_arg().required(true))
		.arg(language_model_arg().required(true))
		.args(C::dict_args().as_ref())
		.group(C::dict_group())
}

fn hill_climb_subcommand<'a,'b, C>() -> App<'a,'b>
where
	C: HillClimb + DictionaryArgs
{
	SubCommand::with_name("hill")
		.about("Hill climb")
		.arg(ciphertext_arg().required(true))
		.arg(language_model_arg().required(true))
		.args(C::dict_args().as_ref())
		.group(C::dict_group())
}

fn decipher_subcommand<'a,'b, C>() -> App<'a,'b>
where
	C: Cipher
{
	SubCommand::with_name("decipher")
		.about("Decipher ciphertext")
		.arg(ciphertext_arg().required(true))
		.arg(key_arg().required(true))
}

fn encipher_subcommand<'a,'b, C>() -> App<'a,'b>
where
	C: Cipher
{
	SubCommand::with_name("encipher")
		.about("Encipher plaintext")
		.arg(plaintext_arg().required(true))
		.arg(key_arg().required(true))
}

pub trait DictionaryArgs {
	fn dict_args<'a,'b>() -> Vec<Arg<'a,'b>>;
	fn dict_group<'a>() -> ArgGroup<'a>;
}

pub trait Cli {
	fn command<'a,'b>() -> App<'a,'b>;
}

impl Cli for Caesar {
	fn command<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name(Caesar::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand::<Self>())
			.subcommand(decipher_subcommand::<Self>())
	}
}

impl DictionaryArgs for Caesar {
	fn dict_args<'a,'b>() -> Vec<Arg<'a,'b>> {
		vec![dict_range_arg()]
	}
	fn dict_group<'a>() -> ArgGroup<'a> {
		ArgGroup::with_name("dictionary")
			.args(&["dict_file"])
			.required(true)
	}
}

impl Cli for Vigenere {
	fn command<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name(Vigenere::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(encipher_subcommand::<Self>())
			.subcommand(decipher_subcommand::<Self>())
			.subcommand(dictionary_attack_subcommand::<Self>())
	}
}

impl DictionaryArgs for Vigenere {
	fn dict_args<'a,'b>() -> Vec<Arg<'a,'b>> {
		vec![dict_file_arg()]
	}
	fn dict_group<'a>() -> ArgGroup<'a> {
		ArgGroup::with_name("dictionary")
			.args(&["dict_file"])
			.required(true)
	}
}

