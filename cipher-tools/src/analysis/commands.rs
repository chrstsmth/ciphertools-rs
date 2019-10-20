use cipher_lib::pallet::alph::*;
use std::convert::TryFrom;
use cipher_lib::analysis::coincidence_count::*;
use cipher_lib::analysis::frequency_analysis::*;
use cipher_tools_lib::parse::*;

pub const FREQUENCY_COMMAND_NAME: &str = "frequency";
pub fn frequency_command(matches: &clap::ArgMatches) {
	let text = text_option(matches);
	let text_alph: Vec<Alph> = text.chars()
		.map(|x| Alph::try_from(x)) .filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect();

	let frequencies = frequency_analysis(&text_alph);
	let mut frequencies: Vec<(&Vec<Alph>, &i32)> = frequencies
		.iter()
		.collect();
	frequencies.sort_by(|a, b| b.1.cmp(a.1));
	let frequencies = frequencies;

	for (ngram, frequency) in frequencies {
		for n in ngram {
			print!("{}", n)
		}
		print!(": ");
		println!("{}", frequency);
	}
}

pub const COINCIDENCE_COUNT_NAME: &str = "coincidences";
pub fn coincidence_count_command(matches: &clap::ArgMatches) {
	let text = text_option(matches);
	let text_alph: Vec<Alph> = text.chars()
		.map(|x| Alph::try_from(x))
		.filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect();

	let coincidences = Coincidences::new(&text_alph);
	print!("{}", coincidences.format(&text_alph));
}

