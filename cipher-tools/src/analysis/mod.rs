extern crate clap;
extern crate enum_map;
extern crate cipher_lib;
extern crate cipher_tools_lib;
extern crate itertools;

mod commands;
mod cli;
mod parse;

use clap::{App, SubCommand, AppSettings};
use commands::*;
use cli::*;
use cipher_tools_lib::cli::*;

fn main() {
	let matches = App::new("Analysis")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(frequency_analysis_subcommand())
		.subcommand(SubCommand::with_name(COINCIDENCE_COUNT_COMMAND_NAME)
			.about("Coincidence count")
			.arg(text_arg().required(true)))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(FREQUENCY_COMMAND_NAME) {
		frequency_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(COINCIDENCE_COUNT_COMMAND_NAME) {
		coincidence_count_command(matches);
	}
}
