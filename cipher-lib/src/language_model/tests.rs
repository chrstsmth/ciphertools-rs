use super::*;

fn langauge_model_abcde() -> LanguageModel {
	let mut l = LanguageModel::new();
	let mut i = "abcde".chars()
		.map(|x| Lang::try_from(x).unwrap());
	l.insert_word(&mut i);
	l
}

fn verify_language_model_abcde(lang: LanguageModel)
{
	let a = lang.head.next.node[usize::from(Lang::A)].as_ref().unwrap();
	let ab = a.next.node[usize::from(Lang::B)].as_ref().unwrap();
	let abc = ab.next.node[usize::from(Lang::C)].as_ref().unwrap();
	let abcd = abc.next.node[usize::from(Lang::D)].as_ref().unwrap();
	let abcde = abcd.next.node[usize::from(Lang::E)].as_ref().unwrap();

	assert!(lang.head.freq == 1);
	assert!(a.freq == 1);
	assert!(ab.freq == 1);
	assert!(abc.freq == 1);
	assert!(abcd.freq == 1);
	assert!(abcde.freq == 1);
}

#[test]
fn test_insert_word_abcde() {
	let l = langauge_model_abcde();
	verify_language_model_abcde(l);
}

#[test]
fn test_serialize_abcde() {
	let l = langauge_model_abcde();
	let serialized = serde_json::to_string(&l).unwrap();
	assert!(serialized == r#"[1,{"a":[1,{"b":[1,{"c":[1,{"d":[1,{"e":[1,{}]}]}]}]}]}]"#);
}

#[test]
fn test_deserialize_abcde() {
	let serialized = r#"[1,{"a":[1,{"b":[1,{"c":[1,{"d":[1,{"e":[1,{}]}]}]}]}]}]"#;
	let l: LanguageModel = serde_json::from_str(&serialized).unwrap();
	verify_language_model_abcde(l);
}

#[test]
fn test_serialize_deserialize_abcde() {
	let l = langauge_model_abcde();
	let serialized = serde_json::to_string(&l).unwrap();
	let deserialized: LanguageModel = serde_json::from_str(&serialized).unwrap();
	let serialized2 = serde_json::to_string(&deserialized).unwrap();
	assert!(serialized2 == r#"[1,{"a":[1,{"b":[1,{"c":[1,{"d":[1,{"e":[1,{}]}]}]}]}]}]"#);
}
