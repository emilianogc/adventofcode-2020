use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub(crate) fn main() {
    lazy_static! {
        static ref MAIN_RE: Regex = Regex::new("\\b(.+?\\b.+?) bags contain (.+)\\.").unwrap();
        static ref RELATIONS_RE: Regex = Regex::new("(\\d+)\\s(.+?\\b.+?) bags?").unwrap();
    }

    let file = read_to_string("src/day7.input").unwrap();
    let mut contained_to_containing: HashMap<String, HashSet<String>> = HashMap::new();
    let mut containing_to_contained: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in file.lines() {
        let rule = MAIN_RE.captures(line).unwrap();
        for contained_bag_parsed in RELATIONS_RE.captures_iter(&rule[2]) {
            let contained_bag_kind: String = contained_bag_parsed[2].into();
            let contained_bag_quantity = contained_bag_parsed[1].parse::<usize>().unwrap();
            let containing_bag: String = rule[1].into();

            contained_to_containing
                .entry(contained_bag_kind.clone())
                .or_default()
                .insert(containing_bag.clone());

            containing_to_contained
                .entry(containing_bag)
                .or_default()
                .insert(contained_bag_kind, contained_bag_quantity);
        }
    }

    let result_1 = part_1(&mut contained_to_containing, "shiny gold".to_string());
    println!("Part 1 reuslt: {}", result_1 - 1);
    let result_2 = part_2(&containing_to_contained, "shiny gold".to_string());
    println!("Part 2 result: {}", result_2 - 1);
}

fn part_1(rules: &mut HashMap<String, HashSet<String>>, bag: String) -> usize {
    let mut to_check = vec![bag];
    let mut visited: HashSet<String> = HashSet::new();

    loop {
        let checking = match to_check.pop() {
            Some(x) => x,
            None => break,
        };

        if visited.contains(&checking) {
            continue;
        }

        visited.insert(checking.clone());

        match rules.remove_entry(&checking) {
            Some((_, entry)) => {
                for value in entry.into_iter() {
                    to_check.push(value)
                }
            }
            None => {}
        }
    }

    visited.len()
}

fn part_2(rules: &HashMap<String, HashMap<String, usize>>, bag: String) -> usize {
    match rules.get(&bag) {
        None => 1,
        Some(bags) => {
            let mut result = 1;
            for bag in bags.iter() {
                let quantity = part_2(rules, bag.0.to_string());
                result += bag.1 * quantity;
            }
            result
        }
    }
}
