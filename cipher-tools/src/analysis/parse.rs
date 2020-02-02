use std::process;
use cli::*;

pub fn ngram_length_option(matches: &clap::ArgMatches) -> Option<usize> {
	Some(ngram_length(matches.value_of(NGRAM_LENGTH_ARG_NAME)?))
}

fn ngram_length<'a>(n: &str) -> usize
{
	match n.parse() {
		Ok(n) => if n > 0 {
			return n;
		}
		_ => ()
	}

	println!("{}: ngram length should be a positive integer", n);
	process::exit(1);
}
