use bit_set::BitSet;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;

const RANGE: i32 = 25;

pub(crate) fn main() {
    let file = read_to_string("src/day9.input").unwrap();
    let mut numbers = Vec::new();
    for line in file.lines() {
        numbers.push(line.parse::<usize>().unwrap());
    }

    let result_1 = part_1(&numbers).unwrap();
    println!("Part 1 result: {}", result_1);
    let result_2 = part_2(result_1, &numbers).unwrap();
    println!("Part 2 result: {}", result_2);
}

fn part_1(numbers: &Vec<usize>) -> Option<usize> {
    let mut sums: HashMap<usize, BitSet> = HashMap::new();

    for (index, number) in numbers.iter().enumerate() {
        let min = max(index as i32 - RANGE, 0);
        for previous in numbers[min as usize..index].iter() {
            sums.entry(number + previous).or_default().insert(index);
        }

        if index as i32 > RANGE {
            let index = sums
                .get(number)
                .and_then(|indexes| indexes.iter().find(|v| *v as i32 > index as i32 - RANGE));
            if index.is_none() {
                return Some(*number);
            }
        }
    }
    None
}

fn part_2(target: usize, numbers: &Vec<usize>) -> Option<usize> {
    let mut result = None;
    for (start, number) in numbers.iter().enumerate() {
        let mut sum = *number;
        for (end, second) in numbers[start + 1..].iter().enumerate() {
            sum += *second;
            if sum == target {
                result = Some((start, end + 1));
                break;
            } else if sum > target {
                break;
            }
        }
        if result.is_some() {
            break;
        }
    }

    let mut max_n = usize::max_value();
    let mut min_n = usize::min_value();

    let indexes = result?;
    for number in numbers.iter().skip(indexes.0).take(indexes.1 + 1) {
        max_n = min(max_n, *number);
        min_n = max(min_n, *number);
    }

    Some(max_n + min_n)
}
