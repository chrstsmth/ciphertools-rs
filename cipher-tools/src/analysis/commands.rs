use cipher_lib::pallet::alph::*;
use std::convert::TryFrom;
use cipher_lib::analysis::*;
use cipher_tools_lib::parse::*;
use cipher_lib::analysis::coincidence_count::*;
use std::collections::BTreeMap;
use std::cmp::Ord;
use std::path::Path;
use std::cmp::Ordering;
use parse::*;
use itertools::Itertools;

pub fn coincidence_count_command(matches: &clap::ArgMatches) {
	let text = text_option(matches);
	let text_alph: Vec<Alph> = text.chars()
		.map(|x| Alph::try_from(x)) .filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect();

	let coincidences = Coincidences::with_length(30, &text_alph);
	println!("{}", coincidences);
}

pub fn frequency_command(matches: &clap::ArgMatches) {
	#[derive(Clone)]
	struct Column<'a, T> {
		val: T,
		file: &'a str,
		i: usize,
	}

	let lang_args = language_model_options(matches);
	let text_args = text_options(matches);
	let mut columns = Vec::new();
	let ngram_length = ngram_length_option(matches).unwrap_or(1);

	match lang_args {
		Some(lang_args) =>
		{
			for l in lang_args {
				columns.push(
					Column {
						val: ngram_frequency_language(ngram_length, &l.value),
						file: l.args[0],
						i: l.i,
					});
			}
		}
		_ => {}
	}

	match text_args {
		Some(text_args) =>
		{
			for t in text_args {
				let text_alph: Vec<Alph> = t.value.chars()
					.map(|x| Alph::try_from(x)) .filter(|x| x.is_ok())
					.map(|x| x.unwrap())
					.collect();
				columns.push(
					Column {
						val: ngram_frequency(ngram_length, &text_alph),
						file: t.args[0],
						i: t.i,
					});
			}
		}
		_ => {}
	}
	columns.sort_by(|a, b| a.i.cmp(&b.i));

	let num_columns = columns.len();
	let mut rows = BTreeMap::<Vec<Alph>, Vec<u32>>::new();
	let mut header = vec![""; num_columns];

	for (i, column) in columns.into_iter().enumerate() {
		header[i] = Path::new(column.file)
			.file_name().unwrap()
			.to_str().unwrap();
		for (key, freq) in column.val {
			(*rows.entry(key).or_insert(vec![0;num_columns]))[i] = freq;
		}
	}

	let mut rows: Vec<_> = rows.iter().collect();
	rows.sort_by(|x, y| {
		for (x, y) in x.1.iter().zip(y.1.iter()) {
			let cmp = y.cmp(&x);
			if cmp != Ordering::Equal {
				return cmp;
			}
		}
		return x.0.cmp(y.0);
	});

	print!(", ");
	header.iter()
		.map(|x| (*x).to_string())
		.intersperse(", ".to_string())
		.fold((), |_, x| print!("{}", x));
	println!("");

	for row in rows {
		let (ngram, freqs) = row;
		for n in ngram {
			print!("{}", n);
		}
		print!(", ");

		freqs.iter()
			.map(|x| (*x).to_string())
			.intersperse(", ".to_string())
			.fold((), |_, x| print!("{}", x));
		println!("");
	}
}

