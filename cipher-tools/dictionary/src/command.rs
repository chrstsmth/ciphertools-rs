use cipher_lib::cipher::Cipher;
use cipher_lib::key::vigenere::*;
use cipher_lib::key::*;
use std::process;

use parse::*;

fn run<I, K, Exit>(keys: I, exit: Exit)
where
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

pub trait Random: Key {
	fn random_command<Exit>(matches: &clap::ArgMatches, exit: Exit)
	where
		Exit: Fn() -> bool;
}

impl Random for VigenereKey {
	fn random_command<Exit>(matches: &clap::ArgMatches, exit: Exit)
	where
		Exit: Fn() -> bool,
	{
		let iter = if let Some(lengths) = lengths_option(matches) {
			Self::random_iterator(lengths)
		} else {
			process::exit(1);
		};

		run(iter, exit);
	}
}

pub fn range_command<C, Exit>(matches: &clap::ArgMatches, exit: Exit)
where
	C: Cipher,
	C::Key: IntoBruteForceIterator + 'static,
	Exit: Fn() -> bool,
{
	let start_key = start_key_option::<C>(&matches);
	let end_key = end_key_option::<C>(&matches);

	//TODO https://github.com/rust-lang/rfcs/pull/2497
	if let Some(start) = start_key.clone() {
		if let Some(end) = end_key.clone() {
			if start > end {
				println!(
					"Start must be less than end: {} > {}",
					start_key.unwrap(),
					end_key.unwrap()
				);
				process::exit(1);
			}
		}
	}

	let start_iter: Box<dyn Iterator<Item = <C as Cipher>::Key>> = match start_key {
		Some(key) => Box::new(key.into_brute_force_iterator()),
		None => Box::new(<C as Cipher>::Key::brute_force_iterator()),
	};

	let iter: Box<dyn Iterator<Item = <C as Cipher>::Key>> = match end_key {
		Some(key) => {
			let key_clone = key.clone();
			Box::new(start_iter.take_while(move |x| *x != key_clone))
		}
		None => Box::new(start_iter),
	};

	run(iter, exit);
}
