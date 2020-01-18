use cipher_lib::pallet::alph::*;
use std::convert::TryFrom;
use cipher_lib::analysis::*;
use cipher_tools_lib::parse::*;
use cipher_lib::analysis::coincidence_count::*;

pub const COINCIDENCE_COUNT_COMMAND_NAME: &str = "coincidence";
pub fn coincidence_count_command(matches: &clap::ArgMatches) {
	let text = text_option(matches);
	let text_alph: Vec<Alph> = text.chars()
		.map(|x| Alph::try_from(x)) .filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect();

	let coincidences = Coincidences::with_length(30, &text_alph);
	println!("{}", coincidences);
}

pub const FREQUENCY_COMMAND_NAME: &str = "frequency";
pub fn frequency_command(matches: &clap::ArgMatches) {
	let text = text_option(matches);
	let text_alph: Vec<Alph> = text.chars()
		.map(|x| Alph::try_from(x)) .filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect();

	let f = frequency(&text_alph);

	for (ngram, frequency) in f {
		println!("{}: {}", ngram, frequency);
	}
}
