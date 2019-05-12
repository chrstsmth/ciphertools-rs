use super::*;

#[test]
fn test_encipher_wehavebeendiscoveredfleeatonce_b() {
	let plaintext = String::from("wehavebeendiscoveredfleeatonce");
	let key = CaesarKey::from('b');
	let ciphertext = Caesar::encipher(&plaintext, &key);
	assert!(ciphertext == "xfibwfcffoejtdpwfsfegmffbupodf");
}

#[test]
fn test_decipher_xfibwfcffoejtdpwfsfegmffbupodf_b() {
	let ciphertext = String::from("xfibwfcffoejtdpwfsfegmffbupodf");
	let key = CaesarKey::from('b');
	let plaintext = Caesar::decipher(&ciphertext, &key);
	assert!(plaintext == "wehavebeendiscoveredfleeatonce");
}
