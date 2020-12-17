use itertools::Itertools;
use std::fs::read_to_string;

pub(crate) fn main() {
    let file = read_to_string("resources/day1.input").expect("input");
    let nums = file
        .lines()
        .map(|line| line.parse::<u32>().expect("input"))
        .collect_vec();

    let result_1 = solution(&nums, 2);
    println!("Part 1 solution: {}", result_1);

    let result_2 = solution(&nums, 3);
    println!("Part 2 solution: {}", result_2);
}

fn solution(nums: &Vec<u32>, n: usize) -> u32 {
    let mut result: Option<(Vec<&u32>, u32)> = None;
    for perm in nums.iter().permutations(n) {
        let sum = perm.iter().fold(0, |acc, &v| acc + *v);
        if sum == 2020 {
            let mult = perm.iter().fold(1, |acc, &v| acc * *v);
            result = Some((perm, mult));
            break;
        }
    }
    let result = result;
    match result {
        Some((_, solution)) => solution,
        None => panic!("No solution found!"),
    }
}
