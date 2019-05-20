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
		impl<S,M> DictionaryAttack<S,M> for #name where
			S: Iterator<Item = Self::Key>,
			M: Model<Self::Key>,
		{
			fn dictionary_attack(ciphertext: &String, dict: S, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>)
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

					results.insert_candidate(can);

					if exit.load(Ordering::SeqCst) {
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
		impl<S,M> BruteForce<S,M> for #name where
			S: Iterator<Item = Self::Key>,
			M: Model<Self::Key>,
		{
			type BruteForceKey = Self::Key;

			fn brute_force(ciphertext: &String, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>)
			{
				Self::dictionary_attack(ciphertext, Self::BruteForceKey::start(), lang, results, exit);
			}

			fn brute_force_from(ciphertext: &String, start: Self::BruteForceKey, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>)
			{
				Self::dictionary_attack(ciphertext, start.into_brute_force_iterator(), lang, results, exit);
			}

			fn brute_force_between(ciphertext: &String, start: Self::BruteForceKey, end: Self::BruteForceKey, lang: LanguageModel, results: &mut M, exit: Arc<AtomicBool>)
			{
				let it = start.into_brute_force_iterator().take_while(|x| *x != end);
				Self::dictionary_attack(ciphertext, it, lang, results, exit);
			}
		}
	};
	TokenStream::from(expanded)
}
