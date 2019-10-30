use cipher_lib::pallet::alph::*;
use std::convert::TryFrom;
use cipher_lib::analysis::coincidence_count::*;
use cipher_lib::analysis::frequency_analysis::*;
use cipher_tools_lib::parse::*;
use colored::*;

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

pub const COINCIDENCE_COUNT_COMMAND_NAME: &str = "coincidences";
pub fn coincidence_count_command(matches: &clap::ArgMatches) {
	let text = text_option(matches);
	let text_alph: Vec<Alph> = text.chars()
		.map(|x| Alph::try_from(x))
		.filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect();

	let coincidences_table = CoincidencesTable::new(&text_alph, 10);
	print_table(&coincidences_table);
}

fn print_coincidence(coincidence: &Coincidence) {
	let (a, b) = coincidence.indices();
	let len = coincidence.len();

	let r = a..a + len;
	let s = b..b + len;
	for (i, c) in coincidence.text().iter().enumerate() {
		let in_r = r.contains(&i);
		let in_s = s.contains(&i);

		let style = if in_r && in_s {
			Some(Color::Magenta)
		} else if in_r {
			Some(Color::Red)
		} else if in_s {
			Some(Color::Blue)
		} else {
			None
		};

		match style {
			Some(col) => print!("{}", c.to_string().color(col)),
			None => print!("{}", c),
		}
	}
}

fn print_table(coincidences_table: &CoincidencesTable) {
	for coincidences in coincidences_table {
		let offset = coincidences.get_offset();

		let mut coincidences = coincidences.into_iter();
		if let Some(coincidence) = coincidences.next() {
			println!("offset: {}", offset);
			print_coincidence(&coincidence);
			println!("");
		}

		for coincidence in coincidences {
			print_coincidence(&coincidence);
			println!("");
		}
	}
}
