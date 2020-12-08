use crate::day8::InstructionKind::{Accumulate, Jump, Noop};
use bit_set::BitSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

enum InstructionKind {
    Jump,
    Noop,
    Accumulate,
}

struct Instruction {
    kind: InstructionKind,
    value: i32,
}

pub(crate) fn main() {
    let file = read_to_string("src/day8.input").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in file.lines() {
        let instruction = parse_instruction(line);
        instructions.push(instruction);
    }

    let result_1 = part_1(instructions);
    println!("Part 1 result: {}", result_1);
}

fn part_1(instructions: Vec<Instruction>) -> i32 {
    let mut acc: i32 = 0;
    let mut pointer: i32 = 0;
    let mut executed = BitSet::new();

    loop {
        if executed.contains(pointer as usize) {
            break acc;
        } else {
            executed.insert(pointer as usize);
        }
        match instructions[pointer as usize] {
            Instruction { kind: Jump, value } => pointer += value,
            Instruction { kind: Noop, .. } => pointer += 1,
            Instruction {
                kind: Accumulate,
                value,
            } => {
                pointer += 1;
                acc += value;
            }
        }
    }
}

fn parse_instruction(line: &str) -> Instruction {
    lazy_static! {
        static ref INSTRUCTION: Regex = Regex::new("(nop|jmp|acc) (\\+|-)(\\d+)").unwrap();
    }

    let captures = INSTRUCTION.captures(line).unwrap();
    let kind = match &captures[1] {
        "jmp" => Jump,
        "acc" => Accumulate,
        "nop" => Noop,
        other => unimplemented!("No such op: {}", other),
    };
    let sign = match &captures[2] {
        "+" => 1,
        "-" => -1,
        other => unimplemented!("Invalid sign for op: {}", other),
    };
    let value = &captures[3].parse::<i32>().unwrap();

    let instruction = Instruction {
        kind,
        value: sign * value,
    };
    instruction
}
