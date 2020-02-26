use cipher_lib::analysis::coincidence_count::*;
use cipher_lib::analysis::*;
use cipher_lib::language_model::*;
use cipher_lib::pallet::alph::*;
use cli::*;
use common::parse::*;
use common::*;
use itertools::Itertools;
use num::Zero;
use parse::*;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::ops::*;
use std::path::Path;
use std::process;

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

pub fn frequency_analyisis_command(matches: &clap::ArgMatches) {
	let lang_args = language_model_options(matches);
	let text_args = text_options(matches);
	let ngram_length = ngram_length_option(matches).unwrap_or(1);

	let table = occurrence_analysis(
		lang_args,
		text_args,
		|x| ngram_frequency_language(ngram_length, x),
		|x| ngram_frequency(ngram_length, x),
	);

	print!("{}", table);
}

pub fn distribution_analysis_command(matches: &clap::ArgMatches) {
	let lang_args = language_model_options(matches);
	let text_args = text_options(matches);
	let ngram_length = ngram_length_option(matches).unwrap_or(1);
	let difference = matches.is_present(DIFFERENCE_ARG_NAME);

	let mut table = occurrence_analysis(
		lang_args,
		text_args,
		|x| ngram_distribution(ngram_frequency_language(ngram_length, x)),
		|x| ngram_distribution(ngram_frequency(ngram_length, x)),
	);

	if difference {
		for row in &mut table.rows {
			let reference = row.1[0];
			for y in row.1.iter_mut().skip(1) {
				*y -= reference;
			}
		}
	}

	print!("{}", table);
}

pub struct OccurrenceTable<'a, N> {
	header: Vec<&'a str>,
	rows: Vec<(Vec<Alph>, Vec<N>)>,
}

impl<'a, N> fmt::Display for OccurrenceTable<'a, N>
where
	N: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, ", ")?;
		self.header
			.iter()
			.map(|x| (*x).to_string())
			.intersperse(", ".to_string())
			.fold(Ok(()), |_, x| write!(f, "{}", x))?;
		writeln!(f, "")?;

		for row in &self.rows {
			let (ngram, freqs) = row;
			for n in ngram {
				write!(f, "{}", n)?;
			}
			write!(f, ", ")?;

			freqs
				.iter()
				.map(|x| (*x).to_string())
				.intersperse(", ".to_string())
				.fold(Ok(()), |_, x| write!(f, "{}", x))?;
			writeln!(f, "")?;
		}
		Ok(())
	}
}

pub fn occurrence_analysis<'a, L, T, N>(
	lang_args: Option<Vec<ParsedArg<'a, LanguageModel>>>,
	text_args: Option<Vec<ParsedArg<'a, String>>>,
	map_from_language: L,
	map_from_text: T,
) -> OccurrenceTable<'a, N>
where
	L: Fn(&LanguageModel) -> HashMap<Vec<Alph>, N>,
	T: Fn(&[Alph]) -> HashMap<Vec<Alph>, N>,
	N: Add + Sub + PartialOrd + Zero + Clone + fmt::Display,
{
	#[derive(Clone)]
	struct Column<'a, T> {
		val: T,
		file: &'a str,
		i: usize,
	}

	let mut columns = Vec::new();

	match lang_args {
		Some(lang_args) => {
			for l in lang_args {
				columns.push(Column {
					val: map_from_language(&l.value),
					file: l.args[0],
					i: l.i,
				});
			}
		}
		_ => {}
	}

	match text_args {
		Some(text_args) => {
			for t in text_args {
				let text_alph: Vec<Alph> = t
					.value
					.chars()
					.map(|x| Alph::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap())
					.collect();
				columns.push(Column {
					val: map_from_text(&text_alph),
					file: t.args[0],
					i: t.i,
				});
			}
		}
		_ => {}
	}
	columns.sort_by(|a, b| a.i.cmp(&b.i));

	let num_columns = columns.len();
	let mut rows = BTreeMap::<Vec<Alph>, Vec<_>>::new();
	let mut header = vec![""; num_columns];

	for (i, column) in columns.into_iter().enumerate() {
		header[i] = Path::new(column.file)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap();
		for (key, freq) in column.val {
			(*rows.entry(key).or_insert(vec![N::zero(); num_columns]))[i] = freq;
		}
	}

	let mut table: OccurrenceTable<'a, N> = OccurrenceTable::<'a, N> {
		header,
		rows: rows.into_iter().collect(),
	};

	table.rows.sort_by(|x, y| {
		for (x, y) in x.1.iter().zip(y.1.iter()) {
			let cmp = y.partial_cmp(&x);
			match cmp {
				Some(cmp) => {
					if cmp != Ordering::Equal {
						return cmp;
					}
				}
				_ => (),
			}
		}
		return x.0.cmp(&y.0);
	});

	return table;
}

pub fn index_of_concidence_command(matches: &clap::ArgMatches) {
	let freq = match language_model_option(matches) {
		Some(language) => frequency_language(&language),
		_ => match text_option(matches) {
			Some(text) => frequency(&string_to_alph(text)),
			_ => process::exit(1),
		},
	};

	let dist = distribution(freq);
	let ic = index_of_coincidence(dist);
	println!("{}", ic);
}
