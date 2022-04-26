use std::fs;

fn main() {
	let filename = "input.txt";

	let mut polymer = fs::read_to_string(filename)
		.expect("Something went wrong");

	// extra line gets added at the end, weird
	polymer = polymer.strip_suffix("\n").unwrap().to_string();

	// let polymer = "dabAcCaCBAcCcaDA".to_string();
	let reacted_polymer = a(polymer);
	println!("result a: {}", reacted_polymer.len());

	let lowest_len = b(String::from_iter(reacted_polymer));
	println!("result b: {}", lowest_len);
}

fn a(polymer: String) -> Vec<char> {
	react(polymer)
}

fn b(polymer: String) -> usize {
	let letters = "abcdefghijklmnopqrstuvwxyz";

	let mut lowest_len = usize::MAX;

	for letter in letters.chars() {
		let p = polymer.replace(letter, "").replace(letter.to_ascii_uppercase(), "");
		let len = a(p).len();
		lowest_len = if len < lowest_len { len } else { lowest_len }
	}
	
	lowest_len
}

// vec stack is much much faster
fn react(polymer: String) -> Vec<char> {
	let chars: Vec<char> = polymer.chars().collect();

	let mut reacted: Vec<char> = vec![];
	reacted.push(chars[0]);

	let mut i = 1;

	while i < chars.len() {
		let stack_top = reacted.last();
		let c2 = chars[i];

		match stack_top {
			Some(&c1) => {
				if is_pair(c1, c2) {
					reacted.pop();
				} else {
					reacted.push(c2);
				}
			},
			None => {
				reacted.push(c2);
			}
		}
		i += 1;
	}

	reacted
}

fn is_pair(c1: char, c2: char) -> bool {
	let opposite = c1.is_uppercase() && c2.is_lowercase() || c1.is_lowercase() && c2.is_uppercase();
	c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && opposite
}
