use std::collections::HashMap;
use std::fs;

fn main() {
	let filename = "input.txt";

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let tokens: Vec<&str> = contents.split_whitespace().collect();
	
	// a(&tokens);
	// b(&tokens);
	b_alt(&tokens);
}

fn a(tokens: &[&str]) {
	let mut two_sum = 0;
	let mut three_sum = 0;

	let mut map: HashMap<char, u32> = HashMap::new();

	for token in tokens {
		for c in token.chars() {
			let count = map.entry(c).or_insert(0);
			*count += 1;
		}

		if map.iter().any(|(_, &v)| v == 2) { two_sum += 1 }
		if map.iter().any(|(_, &v)| v == 3) { three_sum += 1 }

		map.clear();
	}

	println!("{}", two_sum * three_sum);
}

fn b(tokens: &[&str]) {
	for i in 0..tokens.len() {
		for j in i+1..tokens.len() {
			let a = tokens[i];
			let b = tokens[j];

			let mut count = 0;
			let mut commons = "".to_string();

			for (c1, c2) in a.chars().zip(b.chars()) {
				if c1 == c2 { 
					commons.push(c1);
					continue;
				} else { 
					count += 1;
				}

				if count > 1 {
					commons.clear();
					break;
				}
			}
			
			if !commons.is_empty() {
				assert!(commons.len() == a.len() - 1);
				println!("{}", commons);
				return
			}
		}
	}
}

fn b_alt(tokens: &[&str]) {
	for i in 0..tokens.len() {
		for j in i+1..tokens.len() {
			let a = tokens[i];
			let b = tokens[j];

			let commons: String = a.chars().zip(b.chars())
				.filter(|(c1, c2)| c1 == c2)
				.map(|(c, _)| c)
				.collect();

			if commons.len() == a.len() - 1 {
				println!("{}", commons);
				return
			} 
		}
	}
}
