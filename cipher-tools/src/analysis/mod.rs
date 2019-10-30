extern crate clap;
extern crate colored;
extern crate enum_map;
extern crate cipher_lib;
extern crate cipher_tools_lib;

mod commands;

use clap::{App, SubCommand, AppSettings};
use commands::*;
use cipher_tools_lib::cli::*;

fn main() {

	let matches = App::new("Analysis")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(SubCommand::with_name(COINCIDENCE_COUNT_COMMAND_NAME)
			.about("Show coincidences")
			.arg(text_arg().required(true)))
		.subcommand(SubCommand::with_name(FREQUENCY_COMMAND_NAME)
			.about("Frequency analysis")
			.arg(text_arg()))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(COINCIDENCE_COUNT_COMMAND_NAME) {
		coincidence_count_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(FREQUENCY_COMMAND_NAME) {
		frequency_command(matches);
	}
}
