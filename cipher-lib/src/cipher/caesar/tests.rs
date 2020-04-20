use super::*;
use std::str::FromStr;

#[test]
fn test_encipher_wehavebeendiscoveredfleeatonce() {
	let plaintext = String::from("wehavebeendiscoveredfleeatonce");
	let key = CaesarKey::from_str("b").unwrap();
	let ciphertext = Caesar::encipher(&plaintext, &key, &());
	assert!(ciphertext == "xfibwfcffoejtdpwfsfegmffbupodf");
}

#[test]
fn test_decipher_wehavebeendiscoveredfleeatonce() {
	let ciphertext = String::from("xfibwfcffoejtdpwfsfegmffbupodf");
	let key = CaesarKey::from_str("b").unwrap();
	let plaintext = Caesar::decipher(&ciphertext, &key, &());
	assert!(plaintext == "wehavebeendiscoveredfleeatonce");
}

#[test]
fn test_encipher_thequickbrownfoxjumpsoveralazydog() {
	let plaintext = String::from("thequickbrownfoxjumpsoveralazydog");
	let key = CaesarKey::from_str("n").unwrap();
	let ciphertext = Caesar::encipher(&plaintext, &key, &());
	assert!(ciphertext == "gurdhvpxoebjasbkwhzcfbirenynmlqbt");
}

#[test]
fn test_decipher_thequickbrownfoxjumpsoveralazydog() {
	let ciphertext = String::from("gurdhvpxoebjasbkwhzcfbirenynmlqbt");
	let key = CaesarKey::from_str("n").unwrap();
	let plaintext = Caesar::decipher(&ciphertext, &key, &());
	println!("{}", plaintext);
	assert!(plaintext == "thequickbrownfoxjumpsoveralazydog");
}
