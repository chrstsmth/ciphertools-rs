use std::process;

use parse::*;
use cipher_lib::key::*;

fn run<I, K, Exit>(keys: I, exit: Exit) where
	I: Iterator<Item = K>,
	K: Key,
	Exit: Fn() -> bool,
{
	for key in keys {
		println!("{}", key);

		if exit() {
			break;
		}
	}
}

pub fn range<K, Exit>(matches: &clap::ArgMatches, exit: Exit) where
	K: IntoBruteForceIterator + 'static,
	Exit: Fn() -> bool,
{
	let start_key = start_key_option::<K>(&matches);
	let end_key = end_key_option::<K>(&matches);

	//TODO https://github.com/rust-lang/rfcs/pull/2497
	if let Some(start) = start_key.clone() {
		if let Some(end) = end_key.clone() {
			if start > end {
				println!("Start must be less than end: {} > {}", start_key.unwrap(), end_key.unwrap());
				process::exit(1);
			}
		}
	}

	let start_iter = match start_key {
		Some(key) => key.into_brute_force_iterator(),
		None => K::start(),
	};

	let iter: Box<dyn Iterator<Item = K>>
		= match end_key {
		Some(key) => {
			let key_clone = key.clone();
			Box::new(start_iter .take_while(move |x| *x != key_clone))
		}
		None => Box::new(start_iter),
	};

	run(iter, exit);
}

