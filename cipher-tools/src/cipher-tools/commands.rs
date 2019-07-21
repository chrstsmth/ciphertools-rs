macro_rules! encipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("encipher") {
			let plaintext = parse::plaintext(matches).unwrap();
			let key = parse::key::<$Cipher>(matches).unwrap();

			println!("{:}", $Cipher::encipher(&plaintext, &key));
		}
	)
}

macro_rules! decipher {
	($matches:ident, $Cipher:ident) => (
		if let Some(matches) = $matches.subcommand_matches("decipher") {
			let ciphertext = parse::ciphertext(matches).unwrap();
			let key = parse::key::<$Cipher>(matches).unwrap();

			println!("{:}", $Cipher::decipher(&ciphertext, &key));
		}
	)
}

macro_rules! dictionary_attack {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("dictionary") {
			let ciphertext = parse::ciphertext(matches).unwrap();
			let language  = parse::language_model(matches).unwrap();
			let dictionary = parse::dictionary::<$Cipher>(matches).unwrap();

			let mut candidates = Candidates::<$Cipher>::with_capacity(10); let insert_candidate = |c: &Candidate<$Cipher>| {
				if candidates.insert_candidate(c) {
					print!("{}[2J", 27 as char);
					println!("{}", candidates);
				}
			};

			let score = |chars: std::str::Chars| {
				let alph = chars
					.map(|x| Lang::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap());

					let tr = language.traverse();
					score(tr, alph.clone())
			};

			let exit_early = || {
				$exit.load(Ordering::SeqCst)
			};

			$Cipher::dictionary_attack(&ciphertext, dictionary, score, insert_candidate, exit_early);
		}
	)
}

macro_rules! brute_force {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("brute") {
			type BruteForceIter = <<$Cipher as Cipher>::Key as IntoBruteForceIterator>::BruteForceIter;
			type Key = <$Cipher as Cipher>::Key;

			let ciphertext = parse::ciphertext(matches).unwrap();
			let language  = parse::language_model(matches).unwrap();

			let start = match matches.value_of("start") {
				Some(key_str) => {
					match Key::from_str(key_str) {
						Ok(key) => Some(key),
						Err(why) => {
							eprintln!("{}: {}", key_str, why);
							process::exit(1);
						}
					}
				}
				None => None
			};

			let end = match matches.value_of("end") {
				Some(key_str) => {
					match Key::from_str(key_str) {
						Ok(key) => Some(key),
						Err(why) => {
							eprintln!("{}: {}", key_str, why);
							process::exit(1);
						}
					}
				}
				None => None
			};

			let mut candidates = Candidates::<$Cipher>::with_capacity(10);
			let insert_candidate = |c: &Candidate<$Cipher>| {
				if candidates.insert_candidate(c) {
					print!("{}[2J", 27 as char);
					println!("{}", candidates);
				}
			};

			let exit_early = || {
				$exit.load(Ordering::SeqCst)
			};

			let score = |chars: std::str::Chars| {
				let alph = chars
					.map(|x| Lang::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap());

					let tr = language.traverse();
					score(tr, alph)
			};

			if let Some(start) = start {
				if let Some(end) = end {
					<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force_between(&ciphertext, start, end, score, insert_candidate, exit_early);
				} else {
					<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force_from(&ciphertext, start, score, insert_candidate, exit_early);
				}
			} else if let Some(end) = end {
				<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force_to(&ciphertext, end, score, insert_candidate, exit_early);
			} else {
				<$Cipher as BruteForce<BruteForceIter, _, _, _>>::brute_force(&ciphertext, score, insert_candidate, exit_early);
			};
		}
	)
}

macro_rules! hill_climb {
	($matches:ident, $Cipher:ident, $exit:ident) => (
		if let Some(matches) = $matches.subcommand_matches("hill") {
			let ciphertext = parse::ciphertext(matches).unwrap();
			let language = parse::language_model(matches).unwrap();
			let dictionary = parse::dictionary::<$Cipher>(matches).unwrap();

			let mut candidates = Candidates::<$Cipher>::with_capacity(10);
			let insert_candidate = |c: &Candidate<$Cipher>| {
				if candidates.insert_candidate(c) {
					print!("{}[2J", 27 as char);
					println!("{}", candidates);
				}
			};

			let exit_early = || {
				$exit.load(Ordering::SeqCst)
			};

			let score = |chars: std::str::Chars| {
				let alph = chars
					.map(|x| Lang::try_from(x))
					.filter(|x| x.is_ok())
					.map(|x| x.unwrap());

					let tr = language.traverse();
					score(tr, alph)
			};

			$Cipher::hill_climb(&ciphertext, dictionary, score, insert_candidate, exit_early);
		}
	)
}
