#![recursion_limit="1024"]

#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(DictionaryAttack)]
pub fn dictionary_attack(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as DeriveInput);
	impl_dictionary_attack(&ast)
}

fn impl_dictionary_attack(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let expanded = quote! {
		impl<S,M,E> DictionaryAttack<S,M,E> for #name where
			S: Iterator<Item = Self::Key>,
			M: FnMut(Candidate<Self>),
			E: Fn() -> bool,
		{
			fn dictionary_attack(ciphertext: &str, dict: S, lang: LanguageModel, mut candidates: M, exit: E)
			{
				for key in dict {
					let text = #name::decipher(&ciphertext, &key);

					let mut alph_iter = text.chars()
						.map(|x| Lang::try_from(x))
						.filter(|x| x.is_ok())
						.map(|x| x.unwrap());

					let can = Candidate {
						score: lang.score(&mut alph_iter),
						text: text,
						key: key,
					};

					candidates(can);

					if exit() {
						break;
					}
				}
			}
		}
	};
	TokenStream::from(expanded)
}

#[proc_macro_derive(BruteForce)]
pub fn brute_force(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as DeriveInput);
	impl_brute_force(&ast)
}

fn impl_brute_force(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let expanded = quote! {
		impl<S,M,E> BruteForce<S,M,E> for #name where
			S: Iterator<Item = Self::Key>,
			M: FnMut(Candidate<Self>),
			E: Fn() -> bool,
		{
			type BruteForceKey = Self::Key;

			fn brute_force(ciphertext: &str, lang: LanguageModel, candidates: M, exit: E)
			{
				Self::dictionary_attack(ciphertext, Self::BruteForceKey::start(), lang, candidates, exit);
			}

			fn brute_force_from(ciphertext: &str, start: Self::BruteForceKey, lang: LanguageModel, candidates: M, exit: E)
			{
				Self::dictionary_attack(ciphertext, start.into_brute_force_iterator(), lang, candidates, exit);
			}

			fn brute_force_between(ciphertext: &str, start: Self::BruteForceKey, end: Self::BruteForceKey, lang: LanguageModel, candidates: M, exit: E)
			{
				let it = start.into_brute_force_iterator().take_while(|x| *x != end);
				Self::dictionary_attack(ciphertext, it, lang, candidates, exit);
			}
		}
	};
	TokenStream::from(expanded)
}

#[proc_macro_derive(HillClimb)]
pub fn hill_climb(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as DeriveInput);
	impl_hill_climb(&ast)
}

fn impl_hill_climb(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let expanded = quote! {
		impl<S,M,E> HillClimb<S,M,E> for #name where
			S: Iterator<Item = Self::Key>,
			M: FnMut(Candidate<Self>),
			E: Fn() -> bool,
		{
			type MutationKey = Self::Key;

			fn hill_climb(ciphertext: &str, dict: S, lang: LanguageModel, mut candidates: M, exit: E)
			{
				for key in dict {
					let text = #name::decipher(&ciphertext, &key);

					let mut alph_iter = text.chars()
						.map(|x| Lang::try_from(x))
						.filter(|x| x.is_ok())
						.map(|x| x.unwrap());

					let mut best_mutation = Candidate {
						score: lang.score(&mut alph_iter),
						text: text,
						key: key.clone(),
					};
					candidates(best_mutation.clone());

					let mut climbed = true;
					while climbed {
						climbed = false;

						for mutated_key in key.clone().into_mutation_iterator() {
							let text = #name::decipher(&ciphertext, &mutated_key);
							let mut alph_iter = text.chars()
								.map(|x| Lang::try_from(x))
								.filter(|x| x.is_ok())
								.map(|x| x.unwrap());

							let competitor = Candidate {
								score: lang.score(&mut alph_iter),
								text: text,
								key: mutated_key.clone(),
							};
							if competitor > best_mutation {
								best_mutation = competitor;
								climbed = true;
							}

							candidates(best_mutation.clone());

							if exit() {
								return;
							}
						}
					}
				}
			}
		}
	};
	TokenStream::from(expanded)
}
