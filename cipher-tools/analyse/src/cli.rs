use clap::{App, ArgGroup, SubCommand};
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
		.arg(text_arg())
		.arg(language_model_arg())
		.group(
			ArgGroup::with_name("language_model_and_text")
				.args(&[TEXT_ARG_NAME, LANGUAGE_MODEL_ARG_NAME])
				.required(true),
		)
}
