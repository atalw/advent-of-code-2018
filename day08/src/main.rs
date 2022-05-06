use std::fs;
use std::io::{Cursor, Result, Read};

fn main() -> Result<()> {
	// let filename = "test.txt";
	let filename = "input.txt";
	
	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let input: Vec<u8> = contents
		.trim_end()
		.split(" ")
		.map(|x| x.parse().unwrap())
		.collect();

	let len = input.len();

	let mut stream = Cursor::new(input);
	let root = Node::from_flat(&mut stream, len);
	assert!(stream.position() == len as u64);

	a(root.clone());
	b(root);

	Ok(())

}

fn a(root: Node) {
	let flat_nodes = root.flatten_nodes();
	let sum = flat_nodes.iter().fold(0, |acc, x| acc + x.metadata_entires.iter().map(|&x| x as u32).sum::<u32>());
	println!("a sum: {:#?}", sum);
}

fn b(root: Node) {
	let sum = b_helper(root);
	println!("b sum: {}", sum);
}

fn b_helper(root: Node) -> u32 {
	if root.child_count == 0 {
		return root.metadata_entires.iter().map(|&x| x as u32).sum::<u32>()
	} 

	let mut sum = 0;
	for index in root.metadata_entires {
		if index == 0 { continue }
		match root.child_nodes.clone().unwrap().get(index as usize - 1) {
			Some(child) => sum += b_helper(child.clone()),
			None => continue
		}
	}
	sum
}

#[derive(Debug, Clone)]
struct Node {
	child_count: u8,
	metadata_count: u8,
	child_nodes: Option<Vec<Node>>,
	metadata_entires: Vec<u8>
}

impl Node {
	fn from_flat(stream: &mut Cursor<Vec<u8>>, len: usize) -> Node {
		if stream.position() == len as u64 {
			panic!()
		}

		let child_count = read_u8(stream);
		let metadata_count = read_u8(stream);
		assert!(metadata_count > 0);

		let mut children = Vec::new();
		for _ in 0..child_count {
			let n = Node::from_flat(stream, len);
			children.push(n);
		}
		let child_nodes = if !children.is_empty() { Some(children) } else { None };

		let mut metadata_entires: Vec<u8> = Vec::new();
		for _ in 0..metadata_count {
			let entry = read_u8(stream);
			metadata_entires.push(entry);
		}

		Node {
			child_count,
			metadata_count,
			child_nodes,
			metadata_entires,
		}
	}
	fn flatten_nodes(&self) -> Vec<Node> {
		let mut nodes: Vec<Node> = Vec::new();

		let n = Node {
			child_count: self.child_count,
			metadata_count: self.metadata_count,
			child_nodes: None,
			metadata_entires: self.metadata_entires.clone()
		};
		nodes.push(n);

		Node::flatten_nodes_helper(self.clone(), nodes)

	}

	fn flatten_nodes_helper(curr: Node, mut nodes: Vec<Node>) -> Vec<Node> {
		if curr.child_nodes.is_none() {
			nodes.push(curr);
			return nodes
		}

		for child in curr.child_nodes.unwrap() {
			if child.child_nodes.is_some() {
				let mut child_nodes = Node::flatten_nodes_helper(child.clone(), Vec::new());
				let n = Node {
					child_count: child.child_count,
					metadata_count: child.metadata_count,
					child_nodes: None,
					metadata_entires: child.metadata_entires
				};
				nodes.push(n);
				nodes.append(&mut child_nodes);

			} else {
				nodes.push(child);
			}

		}

		nodes
	}
}

fn read_u8(stream: &mut Cursor<Vec<u8>>) -> u8 {
	let mut buf = [0; 1];
	match stream.read_exact(&mut buf) {
		Ok(_) => u8::from_be_bytes(buf),
		Err(e) => panic!("{}", e)
	}
}
