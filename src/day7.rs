use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub(crate) fn main() {
    lazy_static! {
        static ref MAIN_RE: Regex = Regex::new("\\b(.+?\\b.+?) bags contain (.+)\\.").unwrap();
        static ref RELATIONS_RE: Regex = Regex::new("\\d\\s(.+?\\b.+?) bags?").unwrap();
    }

    let file = read_to_string("src/day7.input").unwrap();
    let mut relations: HashMap<String, HashSet<String>> = HashMap::new();

    for line in file.lines() {
        let rule = MAIN_RE.captures(line).unwrap();
        for contained_bag_parsed in RELATIONS_RE.captures_iter(&rule[2]) {
            let contained_bag: String = contained_bag_parsed[1].into();
            let containing_bag: String = rule[1].into();
            let set = relations.entry(contained_bag).or_default();
            set.insert(containing_bag);
        }
    }

    let mut to_check = vec![String::from("shiny gold")];
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

        match relations.remove_entry(&checking) {
            Some((_, entry)) => {
                for value in entry.into_iter() {
                    to_check.push(value)
                }
            }
            None => {}
        }
    }

    println!("Colours that can contain a shiny gold bag: {}", visited.len() - 1);
}
