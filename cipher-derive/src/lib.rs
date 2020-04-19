#![recursion_limit = "1024"]

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
		impl DictionaryAttack for #name where
		{
			fn dictionary_attack<Dict,Can,Exit,Score>(ciphertext: &str, dict: Dict, config: &Self::Config, score: Score, mut candidates: Can, exit: Exit) where
				Dict: Iterator<Item = Self::Key>,
				Can: FnMut(&Candidate<Self>),
				Exit: Fn() -> bool,
				Score: Fn(Chars) -> u32,
			{
				for key in dict {
					let text = #name::decipher(&ciphertext, &key, &config);

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

#[proc_macro_derive(HillClimb)]
pub fn hill_climb(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as DeriveInput);
	impl_hill_climb(&ast)
}

fn impl_hill_climb(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let expanded = quote! {
		impl HillClimb for #name
		{
			type MutationKey = Self::Key;

			fn hill_climb<Dict,Can,Exit,Score>(ciphertext: &str, dict: Dict, config: &Self::Config, score: Score, mut candidates: Can, exit: Exit) where
				Dict: Iterator<Item = Self::Key>,
				Can: FnMut(&Candidate<Self>),
				Exit: Fn() -> bool,
				Score: Fn(Chars) -> u32,
			{
				for key in dict {
					let text = #name::decipher(&ciphertext, &key, &config);

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
							let text = #name::decipher(&ciphertext, &mutated_key, &config);

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
