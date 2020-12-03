use std::io::stdin;

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
        Slope {dx: 1, dy: 1, next_x: 1, next_y: 1, trees: 0},
        Slope {dx: 1, dy: 3, next_x: 1, next_y: 3, trees: 0},
        Slope {dx: 1, dy: 5, next_x: 1, next_y: 5, trees: 0},
        Slope {dx: 1, dy: 7, next_x: 1, next_y: 7, trees: 0},
        Slope {dx: 2, dy: 1, next_x: 2, next_y: 1, trees: 0},
    ];
    let mut xi = 0;

    println!("Enter lines (Ctrl+D to end): ");
    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Err(e) => eprintln!("Failed reading: {}", e),
            _ => {}
        }
        let line: &str = buffer.as_str();
        let size = line.len();

        for slope in &mut slopes {
            if xi == slope.next_x {
                let char = line.chars().nth(slope.next_y % (size - 1));

                match char {
                    Some('.') => {},
                    Some('#') => slope.trees += 1 ,
                    other => panic!("Not a valid char: {:?}", other)
                }

                slope.next_x += slope.dx;
                slope.next_y += slope.dy;
            }
        }

        xi += 1;
    }


    for slope in &slopes {
        println!("Trees found: for {}, {} : {}", slope.dx, slope.dy, slope.trees);
    }
    let result = slopes.iter().fold(1, |acc, v| acc * v.trees);
    println!("Result: {}", result);
}