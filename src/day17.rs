#![feature(type_ascription)]

use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::read_to_string;

lazy_static! {
    static ref THRESHOLD: usize = 4;
}

fn main() -> () {
    let file = read_to_string("resources/day17.input").expect("file input");

    let result_1 = part_1(&file);
    println!("Part 1 result: {}", result_1);
    let result_2 = part_2(&file);
    println!("Part 2 result: {}", result_2);
}

fn part_1(file: &String) -> usize {
    let mut dimension = HashMap::new();
    let lines = file.lines().collect_vec();

    let x_len = lines.len() as isize;
    let y_len = lines[0].len() as isize;

    let mut x_max = x_len / 2;
    let mut y_max = y_len / 2;
    let mut z_max = 0;

    for (x, line) in lines.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            dimension.insert(
                (x as isize - x_len / 2, y as isize - y_len / 2, 0),
                match char {
                    '#' => true,
                    '.' => false,
                    other => panic!("wrong input: {}", other),
                },
            );
        }
    }

    for _ in 0..6 {
        x_max += 1;
        y_max += 1;
        z_max += 1;

        let old = dimension.clone();
        for x in -x_max..=x_max {
            for y in -y_max..=y_max {
                for z in -z_max..=z_max {
                    let active = count_active_neighbors_3(&old, x, y, z);
                    if *old.get(&(x, y, z)).unwrap_or(&false) {
                        dimension.insert((x, y, z), active == 2 || active == 3);
                    } else {
                        dimension.insert((x, y, z), active == 3);
                    }
                }
            }
        }
    }

    return dimension.values().filter(|&x| *x).count();
}

fn count_active_neighbors_3(
    space: &HashMap<(isize, isize, isize), bool>,
    x: isize,
    y: isize,
    z: isize,
) -> usize {
    let mut count = 0;
    for dx in (x - 1)..=(x + 1) {
        for dy in (y - 1)..=(y + 1) {
            for dz in (z - 1)..=(z + 1) {
                let neighbor = (dx, dy, dz);
                if neighbor != (x, y, z) && *space.get(&neighbor).unwrap_or(&false) {
                    count += 1;
                    if count >= *THRESHOLD {
                        return count;
                    }
                }
            }
        }
    }
    return count;
}

fn part_2(file: &String) -> usize {
    let mut dimension = HashMap::new();
    let lines = file.lines().collect_vec();

    let x_len = lines.len() as isize;
    let y_len = lines[0].len() as isize;

    let mut x_max = x_len / 2;
    let mut y_max = y_len / 2;
    let mut z_max = 0;
    let mut w_max = 0;

    for (x, line) in lines.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            dimension.insert(
                (x as isize - x_len / 2, y as isize - y_len / 2, 0, 0),
                match char {
                    '#' => true,
                    '.' => false,
                    other => panic!("wrong input: {}", other),
                },
            );
        }
    }

    for _ in 0..6 {
        x_max += 1;
        y_max += 1;
        z_max += 1;
        w_max += 1;

        let old = dimension.clone();
        for x in -x_max..=x_max {
            for y in -y_max..=y_max {
                for z in -z_max..=z_max {
                    for w in -w_max..=w_max {
                        let active = count_active_neighbors_4(&old, x, y, z, w);
                        if *old.get(&(x, y, z, w)).unwrap_or(&false) {
                            dimension.insert((x, y, z, w), active == 2 || active == 3);
                        } else {
                            dimension.insert((x, y, z, w), active == 3);
                        }
                    }
                }
            }
        }
    }

    return dimension.values().filter(|&x| *x).count();
}

fn count_active_neighbors_4(
    space: &HashMap<(isize, isize, isize, isize), bool>,
    x: isize,
    y: isize,
    z: isize,
    w: isize,
) -> usize {
    let mut count = 0;
    for dx in (x - 1)..=(x + 1) {
        for dy in (y - 1)..=(y + 1) {
            for dz in (z - 1)..=(z + 1) {
                for dw in (w - 1)..=(w + 1) {
                    let neighbor = (dx, dy, dz, dw);
                    if neighbor != (x, y, z, w) && *space.get(&neighbor).unwrap_or(&false) {
                        count += 1;
                        if count >= *THRESHOLD {
                            return count;
                        }
                    }
                }
            }
        }
    }
    return count;
}
