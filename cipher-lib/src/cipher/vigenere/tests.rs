use super::*;
use std::str::FromStr;

#[test]
fn test_encipher_wehavebeendiscoveredfleeatonce_password() {
	let plaintext = String::from("wehavebeendiscoveredfleeatonce");
	let key = VigenereKey::from_str("password").unwrap();
	let ciphertext = Vigenere::encipher(&plaintext, &key, &());
	assert!(ciphertext == "lezsrsshtnvaoqfytrwvbzvhptgfys");
}

#[test]
fn test_decipher_lezsrsshtnvaoqfytrwvbzvhptgfys_password() {
	let ciphertext = String::from("lezsrsshtnvaoqfytrwvbzvhptgfys");
	let key = VigenereKey::from_str("password").unwrap();
	let plaintext = Vigenere::decipher(&ciphertext, &key, &());
	assert!(plaintext == "wehavebeendiscoveredfleeatonce");
}

#[test]
fn test_encipher_thequickbrownfoxjumpsoveralazydog() {
	let plaintext = String::from("thequickbrownfoxjumpsoveralazydog");
	let key = VigenereKey::from_str("ciphertools").unwrap();
	let ciphertext = Vigenere::encipher(&plaintext, &key, &());
	assert!(ciphertext == "vptxyzvypcgyvuvbanaddgxmghprsmrzy");
}

#[test]
fn test_decipher_thequickbrownfoxjumpsoveralazydog() {
	let ciphertext = String::from("vptxyzvypcgyvuvbanaddgxmghprsmrzy");
	let key = VigenereKey::from_str("ciphertools").unwrap();
	let plaintext = Vigenere::decipher(&ciphertext, &key, &());
	assert!(plaintext == "thequickbrownfoxjumpsoveralazydog");
}
