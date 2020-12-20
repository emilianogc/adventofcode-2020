#![feature(str_split_once)]
#![feature(or_patterns)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() -> () {
    let file = read_to_string("resources/day18.input").expect("input file");

    let result_1 = shunting_yard(&file, part_1_precedence);
    println!("Part 1 result: {}", result_1);

    let result_2 = shunting_yard(&file, part_2_precedence);
    println!("Part 2 result: {}", result_2);
}

type OpPrecedenceFn = fn(&mut Vec<Op2>, &mut VecDeque<Op2>, Op2);

fn shunting_yard(file: &String, operator_precedence: OpPrecedenceFn) -> u64 {
    lazy_static! {
        static ref TOKEN_RE: Regex = Regex::new("[0-9]+|[+*()]").unwrap();
    }
    let mut result: u64 = 0;

    for line in file.lines() {
        let mut rest = line;

        let mut output = VecDeque::new();
        let mut operators = Vec::new();

        while !rest.is_empty() {
            let m = TOKEN_RE.find(rest).unwrap();
            let split = rest.split_at(m.end());
            let token = split.0.trim();
            rest = split.1.trim();

            match token {
                "(" => operators.push(Op2::LParen),
                ")" => loop {
                    match operators.pop() {
                        Some(Op2::LParen) | None => break,
                        Some(token) => output.push_back(token),
                    }
                },
                "*" => operator_precedence(&mut operators, &mut output, Op2::Product),
                "+" => operator_precedence(&mut operators, &mut output, Op2::Sum),
                num => {
                    let val = Op2::Val(num.parse().unwrap());
                    output.push_back(val)
                }
            };
        }

        while let Some(token) = operators.pop() {
            output.push_back(token);
        }

        while let Some(front) = output.pop_front() {
            match front {
                Op2::Sum => {
                    if let (Some(Op2::Val(lhs)), Some(Op2::Val(rhs))) =
                        (operators.pop(), operators.pop())
                    {
                        operators.push(Op2::Val(lhs + rhs));
                    }
                }
                Op2::Product => {
                    if let (Some(Op2::Val(lhs)), Some(Op2::Val(rhs))) =
                        (operators.pop(), operators.pop())
                    {
                        operators.push(Op2::Val(lhs * rhs));
                    }
                }
                val => operators.push(val),
            };
        }

        match operators.pop() {
            Some(Op2::Val(val)) => result += val,
            _ => panic!("unexpected"),
        }
    }
    result
}

fn part_1_precedence(operators: &mut Vec<Op2>, output: &mut VecDeque<Op2>, op: Op2) -> () {
    while let Some(Op2::Product | Op2::Sum) = operators.last() {
        output.push_back(operators.pop().unwrap());
    }
    operators.push(op)
}

fn part_2_precedence(operators: &mut Vec<Op2>, output: &mut VecDeque<Op2>, op: Op2) -> () {
    if op == Op2::Product {
        while let Some(Op2::Sum) = operators.last() {
            output.push_back(operators.pop().unwrap());
        }
    }
    operators.push(op)
}

#[derive(Debug, Eq, PartialEq)]
enum Op2 {
    Val(u64),
    LParen,
    Product,
    Sum,
}
