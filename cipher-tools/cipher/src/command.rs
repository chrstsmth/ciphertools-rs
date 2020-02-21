use cipher_lib::cipher::*;

use cipher_lib::candidate::*;
use cipher_lib::language_model::*;
use cipher_lib::pallet::alph::*;
use cipher_lib::score::*;

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
			.map(|x| Alph::try_from(x))
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap());

		let tr = language_model.traverse();
		score(tr, alph)
	}
}

pub fn encipher_command<C: Cipher>(matches: &clap::ArgMatches) {
	let plaintext = plaintext_option(&matches);
	let key = key_option::<C>(&matches);

	println!("{}", <C>::encipher(&plaintext, &key));
}

pub fn decipher_command<C: Cipher>(matches: &clap::ArgMatches) {
	let ciphertext = ciphertext_option(&matches);
	let key = key_option::<C>(&matches);

	println!("{}", <C>::decipher(&ciphertext , &key));
}

pub fn dictionary_attack_command<C, Exit>(matches: &clap::ArgMatches, exit: Exit)
where
	C: DictionaryAttack,
	Exit: Fn() -> bool,
{
	let ciphertext = ciphertext_option(&matches);
	let dictionary = dictionary_option::<C>(&matches);
	let language_model = language_model_option(&matches);

	<C>::dictionary_attack(
		&ciphertext,
		dictionary,
		score_candidate(language_model),
		insert_candidates(),
		exit,
	);
}

pub fn hillclimb_command<C, Exit>(matches: &clap::ArgMatches, exit: Exit)
where
	C: HillClimb,
	Exit: Fn() -> bool,
{
	let ciphertext = ciphertext_option(&matches);
	let dictionary = dictionary_option::<C>(&matches);
	let language_model = language_model_option(&matches);

	<C>::hill_climb(
		&ciphertext,
		dictionary,
		score_candidate(language_model),
		insert_candidates(),
		exit,
	);
}
