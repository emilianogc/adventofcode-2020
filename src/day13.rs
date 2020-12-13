use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::read_to_string;

pub(crate) fn main() -> () {
    let string = read_to_string("resources/day13.input").unwrap();
    let lines = string.lines().collect_vec();
    let departure = lines[0].parse::<u32>().unwrap();
    let schedule = lines[1]
        .split(",")
        .enumerate()
        .flat_map(|v| match v {
            (_, "x") => None,
            (index, other) => Some((index, other.parse::<u32>().unwrap())),
        })
        .collect_vec();

    println!("Schedule: {:?}", schedule);

    let (bus, min_departure) = schedule
        .clone()
        .into_iter()
        .map(|bus| (bus.1, (departure / bus.1 + 1) * bus.1))
        .min_by_key(|v| v.1)
        .unwrap();

    println!("Part 1 result: {}", (min_departure - departure) * bus);

    let result_2 = lines[1].split(',')
        .enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|x: usize| (x - (i + x - 1) % x - 1, x)))
        .fold((0, 1), |(r1, q1), (r2, q2)| crt(r1, q1, r2, q2))
        .0;

    println!("Part 2 result: {}", result_2);
}

fn crt(r1: usize, q1: usize, r2: usize, q2: usize) -> (usize, usize) {
    let mut a = r1;
    let mut b = r2;
    let q = q1 * q2 / gcd(q1, q2);
    loop {
        match a.cmp(&b) {
            Ordering::Less => a += ((b -   a + q1 - 1) / q1) * q1,
            Ordering::Equal => return (a, q),
            Ordering::Greater => b += ((a - b + q2 - 1) / q2) * q2,
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut x, mut y) = if a < b { (a, b) } else { (b, a) };
    while x != 0 {
        let tmp = x;
        x = y % x;
        y = tmp;
    }
    y
}
