use cipher_lib::cipher::*;

use cipher_lib::candidate::*;
use cipher_lib::language_model::*;
use cipher_lib::alphabet::latin::*;
use cipher_lib::score::*;
use cipher_lib::algorithm::*;
use cipher_lib::key::*;

use common::parse::*;
use std::convert::TryFrom;
use std::str::Chars;

fn insert_candidates<C: Cipher>() -> impl FnMut(&Candidate<C>) {
	let mut candidates = Candidates::<C>::with_capacity(10);
	move |c: &Candidate<C>| {
		if candidates.insert_candidate(c) {
			print!("{}[2J", 27 as char);
			println!("{}", candidates);
		}
	}
}

fn score_candidate(language_model: LanguageModel) -> impl Fn(Chars) -> u32 {
	move |chars: std::str::Chars| {
		let alph = chars
			.map(|x| Latin::try_from(x))
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap());

		let tr = language_model.traverse();
		score(tr, alph)
	}
}

pub fn encipher_command<C: Cipher>(matches: &clap::ArgMatches, config: &C::Config) {
	let plaintext = plaintext_option(&matches).unwrap();
	let key = key_option::<C>(&matches).unwrap();

	println!("{}", <C>::encipher(&plaintext, &key, &config));
}

pub fn decipher_command<C: Cipher>(matches: &clap::ArgMatches, config: &C::Config) {
	let ciphertext = ciphertext_option(&matches).unwrap();
	let key = key_option::<C>(&matches).unwrap();

	println!("{}", <C>::decipher(&ciphertext, &key, &config));
}

pub fn dictionary_attack_command<C, Exit>(matches: &clap::ArgMatches, config: &C::Config, exit: Exit)
where
	C: Cipher,
	Exit: Fn() -> bool,
{
	let ciphertext = ciphertext_option(&matches).unwrap();
	let dictionary = dictionary_option::<C>(&matches).unwrap();
	let language_model = language_model_option(&matches).unwrap();

	dictionary_attack::<C,_,_,_,_>(
		&ciphertext,
		dictionary,
		&config,
		score_candidate(language_model),
		insert_candidates(),
		exit,
	);
}

pub fn hillclimb_command<C, Exit>(matches: &clap::ArgMatches, config: &C::Config, exit: Exit)
where
	C: Cipher,
	C::Key: IntoMutationIterator,
	Exit: Fn() -> bool,
{
	let ciphertext = ciphertext_option(&matches).unwrap();
	let dictionary = dictionary_option::<C>(&matches).unwrap();
	let language_model = language_model_option(&matches).unwrap();

	hill_climb::<C,_,_,_,_>(
		&ciphertext,
		dictionary,
		&config,
		score_candidate(language_model),
		insert_candidates(),
		exit,
	);
}
