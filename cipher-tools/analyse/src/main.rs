extern crate cipher_lib;
extern crate clap;
extern crate common;
extern crate enum_map;
extern crate itertools;
extern crate num;

mod cli;
mod command;
mod parse;

use clap::{App, AppSettings, SubCommand};
use cli::*;
use command::*;
use common::cli::*;

fn main() {
	let matches = App::new("Analysis")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(frequency_analysis_subcommand())
		.subcommand(distribution_analysis_subcommand())
		.subcommand(
			SubCommand::with_name(COINCIDENCE_COUNT_COMMAND_NAME)
				.about("Coincidence count")
				.arg(text_arg().required(true)),
		)
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(FREQUENCY_COMMAND_NAME) {
		frequency_analyisis_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(DISTRIBUTION_COMMAND_NAME) {
		distribution_analysis_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(COINCIDENCE_COUNT_COMMAND_NAME) {
		coincidence_count_command(matches);
	}
}
