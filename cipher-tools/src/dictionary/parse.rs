use std::process;

use cipher_lib::key::*;

pub fn start_key_option<K: Key>(matches: &clap::ArgMatches) -> Option<K>
{
	match matches.value_of("start_key") {
		Some(key_str) => Some(key::<K>(matches.value_of(key_str).unwrap())),
		None => None,
	}
}

pub fn end_key_option<K: Key>(matches: &clap::ArgMatches) -> Option<K>
{
	match matches.value_of("end_key") {
		Some(key_str) => Some(key::<K>(matches.value_of(key_str).unwrap())),
		None => None,
	}
}

pub fn len_option(matches: &clap::ArgMatches) -> Option<usize>
{
	match matches.value_of("len") {
		Some(len_str) => {
			match len_str.parse() {
				Ok(len) => Some(len),
				_ => {
					println!("{}: Parse len failed", len_str);
					process::exit(1);
				}
			}
		},
		None => None,
	}
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

