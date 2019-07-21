use clap::{Arg, App, SubCommand, AppSettings};

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;

mod arg {
	use super::*;

	pub fn key<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("key")
			.short("k")
			.value_name("KEY")
			.required(true)
	}

	pub fn ciphertext<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("ciphertext")
			.short("c")
			.value_name("CIPHERTEXT")
			.required(true)
	}

	pub fn plaintext<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("plaintext")
			.short("p")
			.value_name("PLAINTEXT")
			.required(true)
	}
	pub fn language_model<'a,'b>() -> Arg<'a,'b> {
		Arg::with_name("language")
			.short("l")
			.value_name("LANGUAGE")
			.required(true)
	}

	pub fn dictionary<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dictionary")
			.short("d")
			.value_name("DICTIONARY")
			.required(true)
	}

	/*
	 * TODO
	dict-file
	dict-random
	dict-range
	dict-stdin
	*/

	pub fn start<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("start")
			.short("s")
			.value_name("START-KEY")
			.required(false)
	}

	pub fn end<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("end")
			.short("e")
			.value_name("END-KEY")
			.required(false)
	}
}

mod subcommand {
	use super::*;

	pub fn dictionary_attack<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("dictionary")
			.about("Dictionary attack")
			.arg(arg::ciphertext())
			.arg(arg::language_model())
			.arg(arg::dictionary())
	}

	pub fn decipher<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("decipher")
			.about("Decipher ciphertext")
			.arg(arg::ciphertext())
			.arg(arg::key())
	}

	pub fn encipher<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("encipher")
			.about("Encipher plaintext")
			.arg(arg::plaintext())
			.arg(arg::key())
	}

	pub fn brute_force<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("brute")
			.about("Brute force")
			.arg(arg::ciphertext())
			.arg(arg::language_model())
			.arg(arg::start())
			.arg(arg::end())
	}

	pub fn hill_climb<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("hill")
			.about("Hill climb")
			.arg(arg::ciphertext())
			.arg(arg::language_model())
			.arg(arg::dictionary())
	}
}

pub trait Subcommand {
	fn subcommand<'a,'b>() -> App<'a,'b>;
}

impl Subcommand for Vigenere {
	fn subcommand<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name(Vigenere::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(subcommand::encipher())
			.subcommand(subcommand::decipher())
			.subcommand(subcommand::dictionary_attack())
			.subcommand(subcommand::brute_force())
			.subcommand(subcommand::hill_climb())
	}
}

impl Subcommand for Caesar {
	fn subcommand<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name(Caesar::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(subcommand::encipher())
			.subcommand(subcommand::decipher())
			.subcommand(subcommand::brute_force())
	}
}
