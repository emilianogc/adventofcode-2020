use crate::day8::InstructionKind::{Accumulate, Jump, Noop};
use bit_set::BitSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
enum InstructionKind {
    Jump,
    Noop,
    Accumulate,
}

#[derive(Debug, Copy, Clone)]
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

    let result_1 = part_1(&instructions);
    println!("Part 1 result: {}", result_1);
    let result_2 = part_2(
        &instructions,
        State { address: 0, acc: 0 },
        false,
        &mut BitSet::new(),
    );
    println!("Part 2 result: {}", result_2.unwrap());
}

fn part_1(instructions: &[Instruction]) -> i32 {
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

#[derive(Copy, Clone)]
struct State {
    address: usize,
    acc: i32,
}

trait Execute {
    fn execute(self: Self, state: State) -> State;
    fn switch(self: Self) -> Option<Instruction>;
}

impl Execute for Instruction {
    fn execute(self: Self, state: State) -> State {
        match self {
            Instruction {
                kind: Accumulate,
                value,
            } => State {
                address: state.address + 1,
                acc: state.acc + value,
            },
            Instruction { kind: Noop, .. } => State {
                address: state.address + 1,
                acc: state.acc,
            },
            Instruction { kind: Jump, value } => State {
                address: (state.address as i32 + value) as usize,
                acc: state.acc,
            },
        }
    }

    fn switch(self: Self) -> Option<Instruction> {
        match self {
            Instruction {
                kind: Accumulate, ..
            } => None,
            Instruction { kind: Noop, value } => Some(Instruction { kind: Jump, value }),
            Instruction { kind: Jump, value } => Some(Instruction { kind: Noop, value }),
        }
    }
}

fn part_2(
    instructions: &[Instruction],
    state: State,
    switched: bool,
    visited: &mut BitSet,
) -> Option<i32> {
    if state.address >= instructions.len() {
        Some(state.acc)
    } else if visited.contains(state.address) {
        None
    } else {
        visited.insert(state.address);
        let instr = &instructions[state.address];

        let result = if switched || instr.switch().is_none() {
            part_2(instructions, instr.execute(state), switched, visited)
        } else {
            instr
                .switch()
                .and_then(|v| part_2(instructions, v.execute(state), true, visited))
                .or(part_2(instructions, instr.execute(state), false, visited))
        };
        visited.remove(state.address);
        result
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
