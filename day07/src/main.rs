use std::error::Error;
use std::fs;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;


// You know what, I'm stuck on this problem and for some reason I can't get over a block. I'm gonna
// move on and solve this later if I feel like, cause I can. **ashamed**


#[derive(Debug, Clone)]
struct Instruction {
	val: char,
	before: Vec<char>,
}

impl Instruction {
	// fn new(c: char) -> Self {
	//     Instruction { val: c, before: Vec::new() }
	// }

	fn new(c: char, before: char) -> Self {
		Instruction { val: c, before: vec![before] }
	}

	fn update_before(&mut self, c: char) {
		self.before.push(c);
		self.before.sort();
		self.before.reverse();
	}
}

fn main() {
	let filename = "test.txt";
	// let filename = "input.txt";

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let mut lines: Vec<&str> = contents.split("\n").collect();
	// remove empty string
	lines.remove(lines.len()-1);
	let mut instructions: Vec<Instruction> = Vec::new();

	for line in &lines {
		let i: Instruction = line.parse().or_else(|e| {
			println!("{}", line);
			Err(e)
		}).expect("uh oh");
		match instructions.iter_mut().find(|x| x.val == i.val) {
			Some(curr) => curr.update_before(i.before[0]),
			None => instructions.push(i)
		}
	}

	println!("{:#?}", lines);
	println!("{:#?}", instructions);

	// a(&mut instructions);
	a(&instructions);
}

// fn a(instructions: &mut [Instruction]) {
//     let start = find_starting_point(instructions);
//     println!("start: {:?}", start);
//     let mut next_in_line: Vec<char> = start.before.clone();

//     let mut res = "".to_string();

//     res.push(start.val);

//     while !next_in_line.is_empty() {
//         println!("next in line: {:?}", next_in_line);
//         let next_char = next_in_line.pop().unwrap();	
//         println!("next char: {}", next_char);
//         match get_instruction(next_char, instructions) {
//             Some(i) => {
//                 res.push(i.val);
//                 next_in_line.append(&mut i.before);
//                 next_in_line.dedup();
//             },
//             None => {
//                 res.push(next_char);
//                 break
//             }
//         }
//     }

//     println!("{}", res);
// }

fn a(instructions: &[Instruction]) {
	let start = find_starting_point(instructions);
	let mut available_steps = start.before;
	let mut taken_steps = vec![start.val];

	while !available_steps.is_empty() {
		println!("available: {:?}", available_steps);
		let next_step = available_steps.pop().unwrap();
		match get_instruction(next_step, instructions) {
			Some(mut i) => {
				available_steps.append(&mut i.before);
				available_steps.sort();
				available_steps.dedup();
				available_steps.reverse();
			},
			None => {}
		};
		taken_steps.push(next_step);
		available_steps.retain(|x| !taken_steps.contains(x));
	}

	println!("res: {:?}", taken_steps.iter().collect::<String>());
}

fn find_starting_point(instructions: &[Instruction]) -> Instruction {
	for i in 0..instructions.len() {
		let mut is_starting = true;
		for j in i..instructions.len() {
			if instructions[j].before.contains(&instructions[i].val) {
				is_starting = false;
				break
			}
		}

		if is_starting { return instructions[i].clone() }
	}

	panic!("what do i do now");
}

fn get_instruction(c: char, instructions: &[Instruction]) -> Option<Instruction> {
	instructions.iter().find(|&x| c == x.val).cloned()
}

impl FromStr for Instruction {
	type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
		// regex parse here
		
		lazy_static! {
			static ref RE: Regex = Regex::new(r"(?x)
			Step\s(?P<c>\w)\s
			must\sbe\sfinished\sbefore\s
			step\s(?P<before>\w)\s
			can\sbegin.
			").unwrap();
		}

		let caps = RE.captures(s).ok_or("unrecognized instruction")?;

		let c = &caps["c"].chars().nth(0).unwrap();
		let before = &caps["before"].chars().nth(0).unwrap();
		Ok(Instruction::new(*c, *before))
    }
}
