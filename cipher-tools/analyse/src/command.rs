use cipher_lib::analysis::coincidence_count::*;
use cipher_lib::analysis::*;
use common::parse::*;
use common::*;
use std::process;

pub fn coincidence_count_command(matches: &clap::ArgMatches) {
	let text: Vec<char> = text_option(matches).unwrap().chars().collect();

	let coincidences = Coincidences::with_length(30, &text);
	for (i, cs) in coincidences.all_offsets().into_iter().enumerate() {
		print!("{}:", i + 1);
		for c in cs {
			for c in c.text() {
				print!("{}", c);
			}
			print!(" ");
		}
		println!("");
	}
}

pub fn index_of_concidence_command(matches: &clap::ArgMatches) {
	let freq = match language_model_option(matches) {
		Some(language) => Frequencies::from(&language),
		_ => match text_option(matches) {
			Some(text) => Frequencies::from(string_to_alph(text).into_iter()),
			_ => process::exit(1),
		},
	};

	let dist = Distribution::from(freq);
	let ic = index_of_coincidence(dist);
	println!("{}", ic);
}
