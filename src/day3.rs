use std::fs::read_to_string;

#[derive(Debug)]
struct Slope {
    dx: usize,
    dy: usize,

    next_x: usize,
    next_y: usize,

    trees: usize,
}

pub(crate) fn main() {
    let mut slopes = [
        make_slope(1, 1),
        make_slope(1, 3),
        make_slope(1, 5),
        make_slope(1, 7),
        make_slope(2, 1),
    ];

    let file = read_to_string("resources/day3.input").expect("input is defined");
    for (xi, line) in file.lines().enumerate() {
        let size = line.len();
        for slope in &mut slopes {
            if xi == slope.next_x {
                let char = line.chars().nth(slope.next_y % (size));

                match char {
                    Some('.') => {}
                    Some('#') => slope.trees += 1,
                    other => panic!("Not a valid char: {:?}", other),
                }

                slope.next_x += slope.dx;
                slope.next_y += slope.dy;
            }
        }
    }

    for slope in &slopes {
        println!(
            "Trees found: for {}, {} : {}",
            slope.dx, slope.dy, slope.trees
        );
    }
    let result = slopes.iter().fold(1, |acc, v| acc * v.trees);
    println!("Result: {}", result);
}

fn make_slope(dx: usize, dy: usize) -> Slope {
    Slope {
        dx,
        dy,
        next_x: dx,
        next_y: dy,
        trees: 0,
    }
}
