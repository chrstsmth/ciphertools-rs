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

	pub fn dict_file<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dict_file")
			.short("dict_file")
			.value_name("FILE")
			.required(true)
	}

	pub fn dict_random<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dict_random")
			.long("dict_random")
	}

	pub fn dict_range<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dict_range")
			.long("dict_range")
			.value_name("(START, END)")
	}

	pub fn dict_brute<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dict_brute")
			.long("dict_brute")
	}

	pub fn dict_stdin<'a,'b>() -> Arg<'a,'b>
	{
		Arg::with_name("dict_stdin")
			.long("dict_stdin")
	}

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
			.arg(arg::dict_file())
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
			.arg(arg::dict_file())
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
