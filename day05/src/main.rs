use std::fs;

fn main() {
	let filename = "input.txt";

	let mut polymer = fs::read_to_string(filename)
		.expect("Something went wrong");

	// extra line gets added at the end, weird
	polymer = polymer.strip_suffix("\n").unwrap().to_string();

	// let polymer = "dabAcCaCBAcCcaDA".to_string();
	let reacted_polymer = a(polymer);
	println!("len: {}", reacted_polymer.len());

	let lowest_len = b(reacted_polymer);
	println!("lowest len: {}", lowest_len);
}

fn a(polymer: String) -> String {
	let mut reacted = polymer;
	let mut i = 0;

	while i + 1 < reacted.len() {
		let res = react(reacted, i);
		reacted = res.0;
		i = res.1;
	}

	reacted
}

fn b(polymer: String) -> usize {
	let letters = "abcdefghijklmnopqrstuvwxyz";

	let mut lowest_len = usize::MAX;

	for letter in letters.chars() {
		println!("at letter: {}", letter);
		let p = polymer.replace(letter, "").replace(letter.to_ascii_uppercase(), "");
		let len = a(p).len();
		lowest_len = if len < lowest_len { len } else { lowest_len }
	}
	
	lowest_len
}

fn react(mut polymer: String, index: usize) -> (String, usize) {
	let mut left = index;
	let mut right = index + 1;

	loop {
		if !is_pair(polymer.clone(), left, right) { break }
		polymer.remove(right);
		polymer.remove(left);
		if left == 0 { break }
		left -= 1;
		right -= 1;
	}

	(polymer, right)
}

fn is_pair(polymer: String, left_index: usize, right_index: usize) -> bool {
	if let (Some(c1), Some(c2)) = (polymer.chars().nth(left_index), polymer.chars().nth(right_index)) {
		let opposite = c1.is_uppercase() && c2.is_lowercase() || c1.is_lowercase() && c2.is_uppercase();
		c1.clone().to_ascii_lowercase() == c2.clone().to_ascii_lowercase() && opposite
	} else {
		false
	}
}
