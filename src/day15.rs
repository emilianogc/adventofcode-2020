use crate::day15::Num::{First, Twice};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::read_to_string;

pub(crate) fn main() -> () {
    let file = read_to_string("resources/day15.input").unwrap();

    let mut turn = 1;
    let mut numbers_to_turn = HashMap::new();
    let mut last = 0;
    for n in file.split(",") {
        let number = n.parse::<u32>().expect("input is a number");
        match numbers_to_turn.get(&number) {
            None => numbers_to_turn.insert(number, First(turn)),
            Some(First(previous_turn)) => {
                numbers_to_turn.insert(number, Twice(*previous_turn, turn))
            }
            Some(Twice(_, previous_turn)) => {
                numbers_to_turn.insert(number, Twice(*previous_turn, turn))
            }
        };
        last = number;
        turn += 1;
    }

    let result_1 = run(2020, turn, last, &mut numbers_to_turn.clone());
    println!("Part 1 result: {}", result_1);

    let result_2 = run(30000000, turn, last, &mut numbers_to_turn.clone());
    println!("Part 2 result: {}", result_2);
}

fn run(limit: u32, turn: u32, last: u32, numbers_to_turn: &mut HashMap<u32, Num>) -> u32 {
    let mut turn = turn;
    let mut last = last;
    while turn <= limit {
        let spoken = match numbers_to_turn.get(&last) {
            Some(Twice(new_turn, old_turn)) => (old_turn - new_turn),
            Some(First(_)) | None => (0),
        };
        let new_entry = match numbers_to_turn.get(&spoken) {
            Some(Twice(_, new_turn)) => Twice(*new_turn, turn),
            Some(First(previous_turn)) => Twice(*previous_turn, turn),
            None => First(turn),
        };
        numbers_to_turn.insert(spoken, new_entry);
        last = spoken;
        turn += 1;
    }
    last
}

#[derive(Debug, Copy, Clone)]
enum Num {
    First(u32),
    Twice(u32, u32),
}
