use clap::{App, Arg, ArgGroup, SubCommand};
use common::cli::*;

pub const COINCIDENCE_COUNT_COMMAND_NAME: &str = "coincidence";
pub fn coincidence_command_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name(COINCIDENCE_COUNT_COMMAND_NAME)
		.about("Coincidence count")
		.arg(text_arg().required(true))
}

pub const INDEX_OF_COINCIDENCE_COMMAND_NAME: &str = "ic";
pub fn index_of_coincidence_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name(INDEX_OF_COINCIDENCE_COMMAND_NAME)
		.about("Index of coincidence")
		.arg(text_arg().required(true))
}

pub const NGRAMS_COMMAND_NAME: &str = "ngrams";
pub fn ngrams_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name(NGRAMS_COMMAND_NAME)
		.about("ngrams")
		.arg(ngram_length_arg().required(true))
		.arg(text_arg())
		.arg(language_model_arg())
		.group(ArgGroup::with_name("language_model_or_text")
				.args(&[TEXT_ARG_NAME, LANGUAGE_MODEL_ARG_NAME])
				.required(true),
		)
}

pub const NGRAM_LENGTH_ARG_NAME: &str = "ngram";
pub fn ngram_length_arg<'a, 'b>() -> Arg<'a, 'b> {
		Arg::with_name(NGRAM_LENGTH_ARG_NAME)
			.short("n")
			.value_name("LENGTH")
}
