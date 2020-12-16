use std::fs::read_to_string;
use itertools::Itertools;
use regex::Regex;
use bit_set::BitSet;
use std::collections::{HashMap, HashSet};


pub(crate) fn main() -> () {
    let field_regex = Regex::new("(.+?): (\\d+)-(\\d+) or (\\d+)-(\\d+)").expect("valid regex");
    let input = read_to_string("resources/day16.input").expect("input is available");
    let split = input.split("\n\n").collect_vec();

    let mut fields = Vec::new();
    for field in split[0].lines() {
        let capture = field_regex.captures(field).expect("input conforms to regex");
        let mut values = BitSet::new();
        let limit_1 = &capture[2].parse::<usize>().expect("input conforms to spec");
        let limit_2 = &capture[3].parse::<usize>().expect("input conforms to spec");
        for v in *limit_1 ..= *limit_2 { values.insert(v); }
        let limit_3 = &capture[4].parse::<usize>().expect("input conforms to spec");
        let limit_4 = &capture[5].parse::<usize>().expect("input conforms to spec");
        for v in *limit_3 ..= *limit_4 { values.insert(v); }
        fields.push(Field { name: capture[1].to_string(), values });
    }

    let mut my_ticket = Vec::new();
    for num in split[1].lines().skip(1).exactly_one().unwrap().split(",") {
        my_ticket.push(num.parse::<usize>().expect("input conforms to spec"));
    }

    let mut nearby_tickets = Vec::new();
    for nearby_ticket in split[2].lines().skip(1) {
        let mut ticket = Vec::new();
        for num in nearby_ticket.split(",") {
            ticket.push(num.parse::<usize>().expect("input conforms to spec"));
        }
        nearby_tickets.push(ticket);
    };


    let mut sum: u64 = 0;
    nearby_tickets.retain(|ticket| {
        let mut invalid = false;
        for ticket_field in ticket.iter() {
            if !fields.iter().any(|desc_field| desc_field.values.contains(*ticket_field)) {
                invalid = true;
                sum += *ticket_field as u64;
            }
        }
        !invalid
    });
    println!("Part 1 result: {}", sum);

    let transposed = transpose(nearby_tickets);
    let mut matching_fields : HashMap<usize, HashSet<String>> = HashMap::new();
    for (index, indexed_fields) in transposed.iter().enumerate() {
        for field in &fields {
            if indexed_fields.iter().all(|v| field.values.contains(*v)) {
                matching_fields.entry(index).or_default().insert(field.name.clone());
            }
        }
    }

    let mut field_mapping = HashMap::new();
    while !matching_fields.is_empty() {
        let (field_index, located_field) = matching_fields.iter().find(|(_, v)| v.len() == 1).unwrap();

        let field_name = located_field.iter().exactly_one().unwrap().clone();
        field_mapping.insert(field_name.clone(), *field_index);
        for (_, v) in matching_fields.iter_mut() {
            v.remove(&field_name);
        };
        matching_fields.retain(|_, v| !v.is_empty());
    }

    let mut result = 1;
    for (name, index) in field_mapping.iter() {
        if (*name).contains("departure") {
            result *= my_ticket[*index];
        }
    }
    println!("Part 2 result: {}", result);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter()
            .map(|inner| inner[i].clone())
            .collect::<Vec<T>>())
        .collect()
}

#[derive(Clone)]
struct Field {
    name: String,
    values: BitSet,
}