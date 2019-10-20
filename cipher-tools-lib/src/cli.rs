use clap::Arg;

pub fn key_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("key")
		.short("k")
		.value_name("KEY")
}

pub fn ciphertext_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("ciphertext")
		.short("c")
		.value_name("CIPHERTEXT")
}

pub fn plaintext_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("plaintext")
		.short("p")
		.value_name("PLAINTEXT")
}

pub fn text_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("text")
		.short("t")
		.value_name("TEXT")
}

pub fn language_model_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("language")
		.short("l")
		.value_name("LANGUAGE")
}

pub fn dictionary_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name("dictionary")
		.short("d")
		.value_name("DICTIONARY")
}
