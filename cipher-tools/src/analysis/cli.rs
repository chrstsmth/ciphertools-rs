use clap::{App, Arg, SubCommand, ArgGroup};
use cipher_tools_lib::cli::*;

//Difference
//length
//Sort

pub const NGRAM_LENGTH_ARG_NAME: &str = "ngram-length";
pub const FREQUENCY_COMMAND_NAME: &str = "frequency";
pub fn frequency_analysis_subcommand<'a,'b>() -> App<'a,'b>
{
	SubCommand::with_name(FREQUENCY_COMMAND_NAME)
		.about("Frequency analysis")
		.arg(text_arg()
			.multiple(true))
		.arg(language_model_arg()
			.multiple(true))
		.arg(Arg::with_name(NGRAM_LENGTH_ARG_NAME)
			.short("n")
			.value_name("ngram"))
		.group(ArgGroup::with_name("language_model_and_text")
			.args(&[TEXT_ARG_NAME, LANGUAGE_MODEL_ARG_NAME])
			.required(true)
			.multiple(true))
}

pub const COINCIDENCE_COUNT_COMMAND_NAME: &str = "coincidence";
