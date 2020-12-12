use bit_set::BitSet;
use std::fs::read_to_string;

pub(crate) fn main() {
    let file = read_to_string("resources/day6.input").unwrap();
    let mut unions = 0;
    let mut intersections = 0;

    for group in file.split("\n\n") {
        let group_answers: Vec<BitSet> = group
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|answer| {
                let mut selected_answers = BitSet::new();
                for char in answer.chars() {
                    match char {
                        'a'..='z' => {
                            selected_answers.insert(char as usize);
                        }
                        _ => {}
                    };
                }
                selected_answers
            })
            .collect();

        let group_intersection = group_answers
            .clone()
            .into_iter()
            .fold_first(|acc, v| acc.intersection(&v).collect::<BitSet>())
            .unwrap();

        let group_union = group_answers
            .into_iter()
            .fold_first(|acc, v| acc.union(&v).collect::<BitSet>())
            .unwrap();

        intersections += group_intersection.len();
        unions += group_union.len();
    }

    println!("Union sum: {}", unions);
    println!("Intersection sum: {}", intersections);
}
