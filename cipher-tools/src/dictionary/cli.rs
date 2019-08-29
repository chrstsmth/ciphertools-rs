use clap::{Arg, App, SubCommand, AppSettings};

use cipher_lib::key::*;
use cipher_lib::key::vigenere::*;
use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;

pub trait Cli {
	fn command<'a,'b>() -> App<'a,'b>;
}

pub fn start_arg<'a,'b>() -> Arg<'a,'b>
{
	Arg::with_name("start_key")
		.short("s")
		.value_name("START-KEY")
		.required(false)
}

pub fn end_arg<'a,'b>() -> Arg<'a,'b>
{
	Arg::with_name("end_key")
		.short("e")
		.value_name("END-KEY")
		.required(false)
}

pub fn len_arg<'a,'b>() -> Arg<'a,'b>
{
	Arg::with_name("len")
		.short("l")
		.value_name("LENGTH")
}

pub fn range_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name("range")
		.about("Brute force")
		.arg(start_arg())
		.arg(end_arg())
}

trait RandomSubcommand: Key {
	fn random_subcommand<'a,'b>() -> App<'a,'b>;
}

pub fn random_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name("random")
		.about("Random keys")
}

impl RandomSubcommand for VigenereKey {
	fn random_subcommand<'a,'b>() -> App<'a,'b> {
		random_subcommand()
			.arg(len_arg().required(true))
	}
}

impl Cli for Caesar {
	fn command<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name(Caesar::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(range_subcommand())
	}
}

impl Cli for Vigenere {
	fn command<'a,'b>() -> App<'a,'b> {
		SubCommand::with_name(Vigenere::NAME)
			.setting(AppSettings::ArgRequiredElseHelp)
			.subcommand(range_subcommand())
			.subcommand(VigenereKey::random_subcommand())
	}
}

