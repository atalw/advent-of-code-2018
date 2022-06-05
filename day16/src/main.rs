use std::{collections::HashMap, fs};

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum InstructionType {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr
}

fn main() {
    let filename = "input_1.txt";
    // let filename = "test.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let input: Vec<&str> = contents.split('\n').collect();

    let sample_mapping = a(input);
    
    let mut freq_map: HashMap<usize, HashMap<InstructionType, u32>> = HashMap::new();
    for mapping in sample_mapping {
        let instruction_freq = freq_map.entry(mapping.0).or_insert_with(HashMap::new);
        for typ in &mapping.1 {
            let f = instruction_freq.entry(*typ).or_insert(0);
            *f += 1;
        }
    }

    b(freq_map);
}

fn a(input: Vec<&str>) -> Vec<(usize, Vec<InstructionType>)> {
    let mut input = input.clone();
    // sample -> (opcode, possible instrucitons
    let mut res: Vec<(usize, Vec<InstructionType>)> = Vec::new();

    while !input.is_empty() {
        let (before, instruction, after) = read_sample(&mut input);
        let opcode = instruction[0];
        let possible_opcodes = find_opcodes(before.clone(), instruction.clone(), after.clone());
        res.push((opcode, possible_opcodes));
    }

    println!("part 1: {}", res.iter().fold(0, |acc, x| if x.1.len()>=3 { acc + 1 } else { acc }));
    res
}

fn b(freq_map: HashMap<usize, HashMap<InstructionType, u32>>) {
    let mut freq_map = freq_map;
    let mut instruction_map: HashMap<usize, InstructionType> = HashMap::new();

    while !freq_map.is_empty() {
        let inst: Vec<(usize, HashMap<InstructionType, u32>)> = freq_map.iter_mut().filter(|x| x.1.keys().len() == 1).map(|(&x, y)| (x, y.clone())).collect();
        for ins in inst {
            if ins.1.len() == 1 {
                let opcode = ins.0;
                let instr = ins.1.keys().last().unwrap();
                instruction_map.insert(opcode, *instr);
                freq_map.remove(&opcode);
                for m in freq_map.values_mut() {
                    m.retain(|&x, _| x != *instr);
                }
            } else {
                panic!()
            }
        }
    }

    let filename = "input_2.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let input: Vec<&str> = contents.trim_end().split('\n').collect();

    let mut register = vec![0, 0, 0, 0];
    for s in &input[1..] {
        let instruction: Vec<usize> = s.split(' ').map(|x| x.parse().unwrap()).collect();
        b_internal(&mut register, instruction, &instruction_map);
    }

    println!("part 2: {}", register[0]);
}

fn read_sample(input: &mut Vec<&str>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let sample: Vec<&str> = input.drain(0..4).collect();
    let before = sample[0].split(": ").collect::<Vec<&str>>()[1]
        .replace('[', "")
        .replace(']', "")
        .split(", ").map(|x| x.parse().unwrap()).collect();
    let instruction = sample[1].split(' ').map(|x| x.parse().unwrap()).collect();
    let after = sample[2].split(":  ").collect::<Vec<&str>>()[1]
        .replace('[', "")
        .replace(']', "")
        .split(", ").map(|x| x.parse().unwrap()).collect();

    (before, instruction, after)
}

// addr -> register A + register B => register C
// addi -> register A + B => register C
// mulr -> register A * register B => register C
// muli -> register A * B => register C
// banr -> register A & register B => register C
// bani -> register A & B => register C
// borr -> register A | register B => register C
// bori -> register A | B => register C
// setr -> register A [ignore] => [register C = A]
// seti -> A [ignore] => [register C = A]
// gtir -> A register B => [1 if A > B else 0]
// gtri -> register A B => [1  if A > B else 0]
// gtrr -> register A register B => [1 if a > B else 0]
// eqir -> A register B => [1 if A == B else 0]
// eqri -> register A B => [1 if A == B else 0]
// eqrr -> register A register B => [1 if A ==B else 0]
macro_rules! execute_instruction {
    ($type: expr, $register: expr, $instruction: expr) => {{
        let res = match $type {
            InstructionType::addr => $register[$instruction[1]] + $register[$instruction[2]],
            InstructionType::addi => $register[$instruction[1]] + $instruction[2],
            InstructionType::mulr => $register[$instruction[1]] * $register[$instruction[2]],
            InstructionType::muli => $register[$instruction[1]] * $instruction[2],
            InstructionType::banr => $register[$instruction[1]] & $register[$instruction[2]],
            InstructionType::bani => $register[$instruction[1]] & $instruction[2],
            InstructionType::borr => $register[$instruction[1]] | $register[$instruction[2]],
            InstructionType::bori => $register[$instruction[1]] | $instruction[2],
            InstructionType::setr => $register[$instruction[1]],
            InstructionType::seti => $instruction[1],
            InstructionType::gtir => (if $instruction[1] > $register[$instruction[2]] { 1 } else { 0 }),
            InstructionType::gtri => (if $register[$instruction[1]] > $instruction[2] { 1 } else { 0 }),
            InstructionType::gtrr => (if $register[$instruction[1]] > $register[$instruction[2]] { 1 } else { 0 }),
            InstructionType::eqir => (if $instruction[1] == $register[$instruction[2]] { 1 } else { 0 }),
            InstructionType::eqri => (if $register[$instruction[1]] == $instruction[2] { 1 } else { 0 }),
            InstructionType::eqrr => (if $register[$instruction[1]] == $register[$instruction[2]] { 1 } else { 0 }),
        };

        $register[$instruction[3]] = res;
    }};
}

fn find_opcodes(before: Vec<usize>, instruction: Vec<usize>, after: Vec<usize>) -> Vec<InstructionType> {
    let instr_type = [InstructionType::addr, InstructionType::addi, InstructionType::mulr, InstructionType::muli,
        InstructionType::banr, InstructionType::bani, InstructionType::borr, InstructionType::bori, InstructionType::setr, 
        InstructionType::seti, InstructionType::gtir, InstructionType::gtri, InstructionType::gtrr, InstructionType::eqir, 
        InstructionType::eqri, InstructionType::eqrr];

    let mut possible_instructions = Vec::new();

    for typ in instr_type {
        let mut register = before.clone();
        execute_instruction!(typ, &mut register, instruction);
        if register == after { possible_instructions.push(typ) }
    }

    possible_instructions
}

fn b_internal(register: &mut Vec<usize>, instruction: Vec<usize>, map: &HashMap<usize, InstructionType>) {
    let typ = map.get(&instruction[0]).unwrap();
    execute_instruction!(typ, register, instruction);
}

