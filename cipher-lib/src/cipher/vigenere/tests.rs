use crate::test_util::*;
use std::fs::File;
use std::io::*;
use std::io::prelude::*;
use super::*;

#[test]
fn test_encipher_wehavebeendiscoveredfleeatonce_password() {
	let plaintext = String::from("wehavebeendiscoveredfleeatonce");
	let key = VigenereKey::try_from(String::from("password")).unwrap();
	let ciphertext = Vigenere::encipher(&plaintext, &key);
	assert!(ciphertext == "lezsrsshtnvaoqfytrwvbzvhptgfys");
}

#[test]
fn test_decipher_lezsrsshtnvaoqfytrwvbzvhptgfys_password() {
	let ciphertext = String::from("lezsrsshtnvaoqfytrwvbzvhptgfys");
	let key = VigenereKey::try_from(String::from("password")).unwrap();
	let plaintext = Vigenere::decipher(&ciphertext, &key);
	assert!(plaintext == "wehavebeendiscoveredfleeatonce");
}

#[test]
fn test_dictionary_lezsrsshtnvaoqfytrwvbzvhptgfys_password() {
	let resources = cipher_tools_resources().unwrap();
	let dictionary = resources.join("vigenere_dictionary.txt");
	let language = resources.join("alice_language.txt");

	let ciphertext = String::from("lezsrsshtnvaoqfytrwvbzvhptgfys");

	let dictionary_file = File::open(dictionary).unwrap();
	let dictionary = BufReader::new(dictionary_file)
		.lines()
		.map(|x| x.unwrap())
		.map(|x| VigenereKey::try_from(x.clone()).unwrap());

	let language_file = File::open(language).unwrap();
	let language_reader = BufReader::new(language_file);
	let lang: LanguageModel = serde_json::from_reader(language_reader).unwrap();

	let candidates = Vigenere::dictionary_attack(ciphertext, dictionary, 1, lang);
	assert!(candidates[0].text == "wehavebeendiscoveredfleeatonce");
	assert!(candidates[0].key == VigenereKey::try_from(String::from("password")).unwrap());
}
