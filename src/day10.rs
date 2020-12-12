use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Add;

pub(crate) fn main() {
    let file = read_to_string("resources/day10.input").unwrap();
    let mut adapters = Vec::new();
    for line in file.lines() {
        adapters.push(line.parse::<i32>().unwrap());
    }
    let collected: Vec<i32> = adapters.into_iter().collect();

    let result_1 = part_1(
        &mut collected.clone().into_iter().collect(),
        0,
        String::from(""),
    )
    .unwrap();
    println!("Part 1 result: {}", result_1.0 * (result_1.1 + 1));
    let result_2 = part_2(&mut collected.clone()).unwrap();
    println!("Part 2 result: {}", result_2);
}

fn part_1(adapters: &mut HashSet<i32>, current_joltage: i32, indent: String) -> Option<(i32, i32)> {
    if adapters.is_empty() {
        Some((0, 0))
    } else {
        let joltage_difference: Vec<i32> = vec![1, 2, 3];
        let mut iter = joltage_difference.into_iter();
        loop {
            match iter.next() {
                Some(diff) => {
                    let attempt = current_joltage + diff;
                    if adapters.contains(&attempt) {
                        adapters.remove(&attempt);
                        match part_1(adapters, attempt, indent.clone().add("  ")) {
                            Some((diff_1, diff_3)) => {
                                break if diff == 1 {
                                    Some((diff_1 + 1, diff_3))
                                } else if diff == 3 {
                                    Some((diff_1, diff_3 + 1))
                                } else {
                                    Some((diff_1, diff_3))
                                }
                            }
                            None => {
                                adapters.insert(attempt);
                            }
                        }
                    }
                }
                None => break None,
            }
        }
    }
}

fn part_2(adapters: &mut Vec<i32>) -> Option<i64> {
    adapters.sort();
    let mut counts: Vec<i64> = Vec::with_capacity(adapters.len());

    for (index, adapter) in adapters.iter().enumerate() {
        if index == 0 {
            counts.push(1);
        } else {
            let mut count = 0;
            for (j, previous) in adapters[0..index].iter().enumerate() {
                if *adapter - 3 == *previous
                    || *adapter - 2 == *previous
                    || *adapter - 1 == *previous
                {
                    count += counts[j];
                } else {
                }
            }
            if *adapter <= 3 {
                count += 1;
            }
            counts.push(count);
        }
    }

    counts.into_iter().last()
}
