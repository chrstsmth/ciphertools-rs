extern crate cipher_lib;
extern crate clap;
extern crate common;
extern crate enum_map;
extern crate itertools;
extern crate num;

mod cli;
mod command;
mod parse;

use clap::{App, AppSettings};
use cli::*;
use command::*;

fn main() {
	let matches = App::new("Analysis")
		.setting(AppSettings::ArgRequiredElseHelp)
		.subcommand(frequency_analysis_subcommand())
		.subcommand(distribution_analysis_subcommand())
		.subcommand(coincidence_command_subcommand())
		.subcommand(index_of_coincidence_subcommand())
		.get_matches();

	if let Some(matches) = matches.subcommand_matches(FREQUENCY_COMMAND_NAME) {
		frequency_analyisis_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(DISTRIBUTION_COMMAND_NAME) {
		distribution_analysis_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(COINCIDENCE_COUNT_COMMAND_NAME) {
		coincidence_count_command(matches);
	} else if let Some(matches) = matches.subcommand_matches(INDEX_OF_COINCIDENCE_COMMAND_NAME) {
		index_of_concidence_command(matches);
	}
}
