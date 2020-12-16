use crate::day14::Instr::{Mask, Mem};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter;

pub(crate) fn main() -> () {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new("mask = ([01X]+)").unwrap();
        static ref INSTRUCTION_RE: Regex = Regex::new("mem\\[([0-9]+)\\] = ([0-9]+)").unwrap();
    }
    let file = read_to_string("resources/day14.input").unwrap();
    let lines = file.lines().collect_vec();

    let mut instructions = Vec::new();
    for line in lines {
        let instr = if line.starts_with("mask") {
            let mask = MASK_RE.captures(line).unwrap();
            let mask = mask.get(1).unwrap();
            Mask(mask.into())
        } else {
            let captures = INSTRUCTION_RE.captures(line).unwrap();
            let mem = captures[1].parse::<usize>().unwrap();
            let value = captures[2].parse::<u64>().unwrap();
            Mem(mem, value)
        };
        instructions.push(instr);
    }

    let memory = part_1(&instructions);

    println!("Part 1 result: {}", memory.iter().sum::<u64>());

    let memory = part_2(&instructions);

    println!("Part 2 result: {}", memory.iter().sum::<u64>());
}

fn part_1(instructions: &Vec<Instr>) -> Vec<u64> {
    let mut mask_0 = 0;
    let mut mask_1 = 0;
    let mut memory = Vec::new();

    for instr in instructions {
        match *instr {
            Mask(m) => {
                mask_0 = 0;
                mask_1 = 0;

                for (index, char) in m.chars().rev().enumerate() {
                    match char {
                        '0' => mask_0 |= 1 << index,
                        '1' => mask_1 |= 1 << index,
                        'X' => continue,
                        other => panic!("Not a valid mask char: {}", other),
                    }
                }
            }
            Mem(a, v) => {
                if a >= memory.len() {
                    for i in memory.len()..a + 1 {
                        memory.push(0);
                    }
                }
                memory[a] = (v & !mask_0) | mask_1;
            }
        }
    }
    memory
}

fn part_2(instructions: &Vec<Instr>) -> Vec<u64> {
    let mut mask = None;
    let mut memory = HashMap::new();

    for instr in instructions {
        match *instr {
            Mask(m) => mask = Some(m),
            Mem(a, v) => {
                let mut addresses: Vec<String> = vec![String::new()];

                let binary_address = format!("{:b}", a);
                let mask = mask.expect("mask is defined");
                let size_diff = mask.len() - binary_address.len();
                let full_address: String = iter::repeat('0')
                    .take(size_diff)
                    .chain(binary_address.chars())
                    .collect();

                for (mc, ac) in itertools::zip(mask.chars(), full_address.chars()) {
                    match mc {
                        '0' => {
                            for addr in addresses.iter_mut() {
                                addr.push(ac)
                            }
                        }
                        '1' => {
                            for addr in addresses.iter_mut() {
                                addr.push('1')
                            }
                        }
                        'X' => {
                            let mut copy = addresses.clone();

                            for addr in copy.iter_mut() {
                                addr.push('0')
                            }
                            for addr in addresses.iter_mut() {
                                addr.push('1')
                            }

                            addresses.append(&mut copy);
                        }
                        _ => unreachable!(),
                    }
                }

                for addr_str in addresses {
                    let address = usize::from_str_radix(addr_str.as_str(), 2).unwrap();
                    memory.insert(address, v);
                }
            }
        }
    }
    memory.values().map(|x| *x).collect_vec()
}

enum Instr<'a> {
    Mask(&'a str),
    Mem(usize, u64),
}
