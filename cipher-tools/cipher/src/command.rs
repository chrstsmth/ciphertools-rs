use cipher_lib::key::any_key::*;
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

fn score_candidate<'a>(language_model: &'a LanguageModel) -> impl Fn(Chars) -> u32 + 'a {
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

pub fn dictionary_attack_command<C>(matches: &clap::ArgMatches, config: C::Config)
where
	C: Cipher,
{
	let ciphertext = ciphertext_option(&matches).unwrap();
	let dictionary = dictionary_option::<C>(&matches).unwrap();
	let language_model = language_model_option(&matches).unwrap();

	let candidates = dictionary_attack::<C,_,_>(
		&ciphertext,
		dictionary,
		config,
		score_candidate(&language_model));

	consume_candidates(candidates);
}

pub fn hillclimb_command<C>(matches: &clap::ArgMatches, config: &C::Config)
where
	C: Cipher,
	C::Key: IntoMutationIterator,
	<C::Key as TryFrom<AnyKey>>::Error: std::fmt::Debug, // for unrap()
{
	let ciphertext = ciphertext_option(&matches).unwrap();
	let dictionary = dictionary_option::<C>(&matches).unwrap();
	let language_model = language_model_option(&matches).unwrap();

	let candidates = dictionary.map(|seed_key|
		hill_climb::<C,_>(
			&ciphertext,
			seed_key,
			config,
			score_candidate(&language_model)))
		.flatten();

	consume_candidates(candidates);
}

fn consume_candidates<Cs>(candidates: Cs)
where
	Cs: Iterator<Item = Candidate>
{
	let intermediates = Candidates::intermediates(10, candidates);
	for cs in intermediates {
		print!("{}[2J", 27 as char);
		for c in cs {
			print!("{}", c);
		}
		println!();
	}
}
