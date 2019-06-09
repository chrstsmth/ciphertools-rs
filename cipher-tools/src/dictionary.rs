extern crate clap;
extern crate cipher_lib;

use std::process;

use cipher_lib::cipher::*;
use cipher_lib::cipher::vigenere::*;
use cipher_lib::cipher::caesar::*;
use cipher_lib::key::*;

use std::str::FromStr;

use clap::{Arg, ArgGroup, App, SubCommand, AppSettings, ArgMatches};

mod subcommand {
	use super::*;

	pub fn brute_force<'a,'b>() -> App<'a,'b>
	{
		SubCommand::with_name("brute")
			.about("Brute force")
			.arg(Arg::with_name("start")
				.short("s")
				.value_name("START-KEY")
				.required(false))
			.arg(Arg::with_name("end")
				.short("e")
				.value_name("END-KEY")
				.required(false))
	}

}

fn vigenere_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name(Vigenere::NAME)
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(subcommand::brute_force())
}

fn caesar_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name(Caesar::NAME)
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(subcommand::brute_force())
}

macro_rules! brute_force {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("brute") {
			type Key = <$Cipher as Cipher>::Key;

			let start = match matches.value_of("start") {
				Some(key_str) => {
					match Key::from_str(key_str) {
						Ok(key) => Some(key),
						Err(why) => {
							eprintln!("{}: {}", key_str, why);
							process::exit(1);
						}
					}
				}
				None => None
			};

			let end = match matches.value_of("end") {
				Some(key_str) => {
					match Key::from_str(key_str) {
						Ok(key) => Some(key),
						Err(why) => {
							eprintln!("{}: {}", key_str, why);
							process::exit(1);
						}
					}
				}
				None => None
			};

			if let Some(start) = start {
				if let Some(end) = end {
					run::<_, $Cipher>(start.into_brute_force_iterator().take_while(|x| *x != end));
				} else {
					run::<_, $Cipher>(start.into_brute_force_iterator());
				}
			} else {
				run::<_, $Cipher>(<Key as IntoBruteForceIterator>::start());
			};
		}
	)
}

fn run<I, C>(keys: I) where
	I: Iterator<Item = C::Key>,
	C: Cipher,
{
	for key in keys {
		println!("{}", key);
	}
}

fn main() {
	let matches = App::new("Dictionary")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(vigenere_subcommand())
		.subcommand(caesar_subcommand())
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(Vigenere::NAME) {
		brute_force!(matches, Vigenere, exit);
	} else if let Some(matches) = matches.subcommand_matches(Caesar::NAME) {
		brute_force!(matches, Caesar, exit);
	}
}
