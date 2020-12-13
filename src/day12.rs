use crate::day12::Step::{East, Forward, Left, North, Right, South, West};
use std::fs::read_to_string;

pub(crate) fn main() -> () {
    let file = read_to_string("resources/day12.input").unwrap();
    let mut steps = Vec::new();
    for line in file.lines() {
        let step = match line.split_at(1) {
            ("N", n) => North(n.parse::<i32>().unwrap()),
            ("S", n) => South(n.parse::<i32>().unwrap()),
            ("E", n) => East(n.parse::<i32>().unwrap()),
            ("W", n) => West(n.parse::<i32>().unwrap()),
            ("L", n) => Left(n.parse::<i32>().unwrap()),
            ("R", n) => Right(n.parse::<i32>().unwrap()),
            ("F", n) => Forward(n.parse::<i32>().unwrap()),
            other => unimplemented!("Invalid step: {:?}", other),
        };
        steps.push(step);
    }

    let result_1 = part_1(&steps);
    println!("Part 1 result: {}", result_1);
    let result_2 = part_2(&steps);
    println!("Part 2 result: {}", result_2);
}

#[derive(Debug)]
enum Step {
    North(i32),
    South(i32),
    East(i32),
    West(i32),

    Left(i32),
    Right(i32),

    Forward(i32),
}

fn part_1(steps: &Vec<Step>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut heading: i32 = 0;

    for step in steps.iter() {
        match step {
            East(v) => x += *v,
            West(v) => x -= *v,
            North(v) => y += *v,
            South(v) => y -= *v,
            Left(v) => heading += *v,
            Right(v) => heading += 360 - *v,
            Forward(v) => {
                let angle = angles(heading);
                x += *v * angle.0;
                y += *v * angle.1;
            }
        }
    }

    x.abs() + y.abs()
}

fn angles(angle: i32) -> (i32, i32) {
    match angle % 360 {
        0 => (1, 0),
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        other => unimplemented!("Invalid angle: {}", other),
    }
}

fn part_2(steps: &Vec<Step>) -> i32 {
    let mut ship_x: i32 = 0;
    let mut ship_y: i32 = 0;

    let mut way_x: i32 = 10;
    let mut way_y: i32 = 1;

    for step in steps.iter() {
        match step {
            East(v) => way_x += *v,
            West(v) => way_x -= *v,
            North(v) => way_y += *v,
            South(v) => way_y -= *v,

            Left(v) => (way_x, way_y) = rotate(*v, way_x, way_y),
            Right(v) => (way_x, way_y) = rotate(360 - *v, way_x, way_y),

            Forward(v) => {
                ship_x += way_x * *v;
                ship_y += way_y * *v;
            }
        }
    }

    ship_x.abs() + ship_y.abs()
}

fn rotate(angle: i32, x: i32, y: i32) -> (i32, i32) {
    match angle {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        360 => (x, y),
        other => unimplemented!("Invalid angle: {}", other),
    }
}
