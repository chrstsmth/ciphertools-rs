use cipher_lib::analysis::coincidence_count::*;
use cipher_lib::analysis::frequency::*;
use cipher_lib::analysis::distribution::*;
use cipher_lib::analysis::ngram_distribution::*;
use cipher_lib::analysis::statistics::*;
use common::parse::*;
use common::*;
use crate::parse::*;
use crate::cli::*;

pub fn coincidence_count_command(matches: &clap::ArgMatches) {
	let text: Vec<char> = text_option(matches).unwrap().chars().collect();

	let coincidences = Coincidences::with_length(30, &text);
	for (i, cs) in coincidences.all_offsets().into_iter().enumerate() {
		print!("{}:", i + 1);
		for c in cs {
			for c in c.text() {
				print!("{}", c);
			}
			print!(" ");
		}
		println!("");
	}
}

pub fn index_of_concidence_command(matches: &clap::ArgMatches) {
	let text = text_option(matches).unwrap();
	let freq = Frequencies::from(string_to_alph(text).into_iter());
	let dist = Distribution::from(freq);
	let ic = index_of_coincidence(dist);
	println!("{}", ic);
}

pub fn ngrams_command(matches: &clap::ArgMatches) {
	let lang = language_model_option(matches).unwrap();
	let ngram_length = ngram_length_option(matches).unwrap();

	let freq = lang.ngram_frequencies(ngram_length);
	let dist = NgramDistribution::from(freq);

	let mut dist = dist
		.into_iter()
		.map(|(ngram,freq)| (ngram, freq * 100.0))
		.collect::<Vec<(_,_)>>();

	dist.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

	for (ngram, freq) in dist {
		print!("{}: ", freq);
		for n in ngram {
			print!("{}", n);
		}
		println!()
	}
}
