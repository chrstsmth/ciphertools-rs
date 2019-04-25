
pub struct VigenereKey(pub String);

impl From<&str> for VigenereKey {
	fn from(key: &str) -> VigenereKey
	{
		VigenereKey(String::from(key))
	}
}

impl From<String> for VigenereKey {
	fn from(key: String) -> VigenereKey
	{
		VigenereKey(key)
	}
}
