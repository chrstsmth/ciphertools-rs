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
		impl<S,M,E,J> DictionaryAttack<S,M,E,J> for #name where
			S: Iterator<Item = Self::Key>,
			M: FnMut(&Candidate<Self>),
			E: Fn() -> bool,
			J: Fn(Chars) -> u32,
		{
			fn dictionary_attack(ciphertext: &str, dict: S, score: J, mut candidates: M, exit: E)
			{
				for key in dict {
					let text = #name::decipher(&ciphertext, &key);

					let can = Candidate {
						score: score(text.chars()),
						text: text,
						key: key,
					};

					candidates(&can);

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
		impl<S,M,E,J> BruteForce<S,M,E,J> for #name where
			S: Iterator<Item = Self::Key>,
			M: FnMut(&Candidate<Self>),
			E: Fn() -> bool,
			J: Fn(Chars) -> u32,
		{
			type BruteForceKey = Self::Key;

			fn brute_force(ciphertext: &str, score: J, candidates: M, exit: E)
			{
				Self::dictionary_attack(ciphertext, Self::BruteForceKey::start(), score, candidates, exit);
			}

			fn brute_force_from(ciphertext: &str, start: Self::BruteForceKey, score: J, candidates: M, exit: E)
			{
				Self::dictionary_attack(ciphertext, start.into_brute_force_iterator(), score, candidates, exit);
			}

			fn brute_force_between(ciphertext: &str, start: Self::BruteForceKey, end: Self::BruteForceKey, score: J, candidates: M, exit: E)
			{
				let it = start.into_brute_force_iterator().take_while(|x| *x != end);
				Self::dictionary_attack(ciphertext, it, score, candidates, exit);
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
		impl<S,M,E,J> HillClimb<S,M,E,J> for #name where
			S: Iterator<Item = Self::Key>,
			M: FnMut(&Candidate<Self>),
			E: Fn() -> bool,
			J: Fn(Chars) -> u32,
		{
			type MutationKey = Self::Key;

			fn hill_climb(ciphertext: &str, dict: S, score: J, mut candidates: M, exit: E)
			{
				for key in dict {
					let text = #name::decipher(&ciphertext, &key);

					let mut best_mutation = Candidate {
						score: score(text.chars()),
						text: text,
						key: key.clone(),
					};
					candidates(&best_mutation);

					let mut climbed = true;
					while climbed {
						climbed = false;

						for mutated_key in key.clone().into_mutation_iterator() {
							let text = #name::decipher(&ciphertext, &mutated_key);

							let competitor = Candidate {
								score: score(text.chars()),
								text: text,
								key: mutated_key.clone(),
							};
							if competitor > best_mutation {
								best_mutation = competitor;
								climbed = true;
							}

							candidates(&best_mutation);

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
