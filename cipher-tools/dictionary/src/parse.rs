use std::process;

use cipher_lib::key::*;

pub fn start_key_option<K: Key>(matches: &clap::ArgMatches) -> Option<K>
{
	match matches.value_of("start_key") {
		Some(key_str) => Some(key::<K>(key_str)),
		None => None,
	}
}

pub fn end_key_option<K: Key>(matches: &clap::ArgMatches) -> Option<K>
{
	match matches.value_of("end_key") {
		Some(key_str) => Some(key::<K>(key_str)),
		None => None,
	}
}

pub fn lengths_option(matches: &clap::ArgMatches) -> Option<Vec<usize>>
{
	let lengths_str = match matches.value_of("lengths") {
		Some(lengths_str) => lengths_str,
		None => return None,
	};

	let lengths: Vec<usize> = lengths_str.split(',')
		.filter(|x| !x.is_empty())
		.map(|x| x.parse()
			.unwrap_or_else(|_| {
				println!("Failed to parse length: {}", x);
				process::exit(1);
			}))
		.collect();

	if lengths.is_empty() {
		println!("No lengths provided: {}", lengths_str);
		process::exit(1);
	}

	Some(lengths)
}

pub fn key<K: Key>(key_str: &str) -> K
{
	match K::from_str(key_str) {
		Ok(key) => key,
		_ => {
			println!("{}: Parse key failed", key_str);
			process::exit(1);
		}
	}
}
