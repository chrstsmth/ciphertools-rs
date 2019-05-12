use super::*;

#[test]
fn test_encipher_wehavebeendiscoveredfleeatonce_b() {
	let plaintext = String::from("wehavebeendiscoveredfleeatonce");
	let key = CaesarKey::try_from(String::from("b")).unwrap();
	let ciphertext = Caesar::encipher(&plaintext, &key);
	assert!(ciphertext == "xfibwfcffoejtdpwfsfegmffbupodf");
}

#[test]
fn test_decipher_xfibwfcffoejtdpwfsfegmffbupodf_b() {
	let ciphertext = String::from("xfibwfcffoejtdpwfsfegmffbupodf");
	let key = CaesarKey::try_from(String::from("b")).unwrap();
	let plaintext = Caesar::decipher(&ciphertext, &key);
	assert!(plaintext == "wehavebeendiscoveredfleeatonce");
}
