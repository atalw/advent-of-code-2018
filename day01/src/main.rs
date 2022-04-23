use std::collections::HashSet;
use std::fs;

fn main() {
	let filename = "input.txt";

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let tokens: Vec<i32> = contents.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

	a(&tokens);
	b(&tokens);
}

fn a(tokens: &[i32]) {
	let mut freq = 0;

	for token in tokens {
		freq += token;
	}

	println!("{:?}", freq);
}

fn b(tokens: &[i32]) {
	let mut set: HashSet<i32> = HashSet::new();
	let mut freq = 0;
	let mut index = 0;

	loop {
		freq += tokens[index];

		match set.insert(freq) {
			true => {
				index += 1;
				if index == tokens.len() { index = 0 };
			},
			false => {
				println!("{}", freq); 
				break 
			}
		}
	}
}
