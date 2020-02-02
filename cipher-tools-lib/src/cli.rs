use clap::Arg;

pub const KEY_ARG_NAME: &str = "key";
pub fn key_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name(KEY_ARG_NAME)
		.short("k")
		.value_name("KEY")
}

pub const CIPHERTEXT_ARG_NAME: &str = "ciphertext";
pub fn ciphertext_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name(CIPHERTEXT_ARG_NAME)
		.short("c")
		.value_name("CIPHERTEXT")
}

pub const PLAINTEXT_ARG_NAME: &str = "plaintext";
pub fn plaintext_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name(PLAINTEXT_ARG_NAME)
		.short("p")
		.value_name("PLAINTEXT")
}

pub const TEXT_ARG_NAME: &str = "text";
pub fn text_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name(TEXT_ARG_NAME)
		.short("t")
		.value_name("TEXT")
}

pub const LANGUAGE_MODEL_ARG_NAME: &str = "language";
pub fn language_model_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name(LANGUAGE_MODEL_ARG_NAME)
		.short("l")
		.value_name("LANGUAGE")
}

pub const DICTIONARY_ARG_NAME: &str = "dictionary";
pub fn dictionary_arg<'a,'b>() -> Arg<'a,'b> {
	Arg::with_name(DICTIONARY_ARG_NAME)
		.short("d")
		.value_name("DICTIONARY")
}
