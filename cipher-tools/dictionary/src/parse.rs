use cipher_lib::cipher::Cipher;
use common::parse;
use std::process;

pub fn start_key_option<'a, C: Cipher>(matches: &clap::ArgMatches) -> Option<C::Key> {
	match matches.value_of("start_key") {
		Some(key_str) => Some(parse::key::<C>(key_str)),
		None => None,
	}
}

pub fn end_key_option<'a, C: Cipher>(matches: &clap::ArgMatches) -> Option<C::Key> {
	match matches.value_of("end_key") {
		Some(key_str) => Some(parse::key::<C>(key_str)),
		None => None,
	}
}

pub fn lengths_option(matches: &clap::ArgMatches) -> Option<Vec<usize>> {
	let lengths_str = match matches.value_of("lengths") {
		Some(lengths_str) => lengths_str,
		None => return None,
	};

	let lengths: Vec<usize> = lengths_str
		.split(',')
		.filter(|x| !x.is_empty())
		.map(|x| {
			x.parse().unwrap_or_else(|_| {
				println!("Failed to parse length: {}", x);
				process::exit(1);
			})
		})
		.collect();

	if lengths.is_empty() {
		println!("No lengths provided: {}", lengths_str);
		process::exit(1);
	}

	Some(lengths)
}
