
use cipher_lib::cipher::*;
use parse::*;

use cipher_lib::candidate::*;
use cipher_lib::language_model::*;
use cipher_lib::pallet::lang::*;
use cipher_lib::score::*;

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
			.map(|x| Lang::try_from(x))
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap());

			let tr = language_model.traverse();
			score(tr, alph)
	}
}

pub fn encipher<C: Cipher>(matches: &clap::ArgMatches)
{
	let ciphertext = ciphertext_option(&matches);
	let key = key_option::<C>(&matches);

	println!("{}", <C>::encipher(&ciphertext, &key));
}

pub fn decipher<C: Cipher>(matches: &clap::ArgMatches)
{
	let plaintext = plaintext_option(&matches);
	let key = key_option::<C>(&matches);

	println!("{}", <C>::decipher(&plaintext, &key));
}

pub fn dictionary_attack<C, Exit>(matches: &clap::ArgMatches, exit: Exit) where
	C: DictionaryAttack + DictionaryOption,
	Exit: Fn() -> bool,
{
	let ciphertext = ciphertext_option(&matches);
	let dictionary = C::dictionary_option(&matches);
	let language_model = language_model_option(&matches);

	<C>::dictionary_attack(&ciphertext,
		dictionary,
		score_candidate(language_model),
		insert_candidates(),
		exit);
}