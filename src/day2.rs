use std::fs::read_to_string;
use text_io::scan;

fn main() {
    let mut correct1 = 0;
    let mut correct2 = 0;

    let file = read_to_string("resources/day2.input").expect("input is provided");
    for line in file.lines() {
        let (a, b, c, pass): (usize, usize, char, String);
        scan!(line.bytes() => "{}-{} {}: {}", a, b, c, pass);

        let count = pass.chars().filter(|cc| c == *cc).count();
        if (count >= a) & (count <= b) {
            correct1 += 1
        }

        if (pass.chars().nth(a - 1).unwrap() == c) ^ (pass.chars().nth(b - 1).unwrap() == c) {
            correct2 += 1
        }
    }
    println!("Amount of correct passwords (step 1): {}", correct1);
    println!("Amount of correct passwords (step 2): {}", correct2);
}
