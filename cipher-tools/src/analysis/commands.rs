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

	let coincidence_table = CoincidencesTable::new(&text_alph);
	let mut scores = vec![0; coincidence_table.len()];

	for (coincidences, s) in coincidence_table.iter().zip(scores.iter_mut()) {
		for coincidence in &coincidences {
			*s += coincidence.len();
		}
	}

	for (i, s) in scores.iter().enumerate() {
		println!("{}: {}", i + 1, s);
	}
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
