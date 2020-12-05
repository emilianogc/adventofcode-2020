use std::io::{stdin, stdout, Write};

use itertools::Itertools;
use text_io::read;

pub(crate) fn main() {
    let mut lines = Vec::new();
    println!("Enter numbers (Ctrl+D to end): ");
    loop {
        let mut buffer = String::new();
        let bytes = stdin().read_line(&mut buffer).unwrap();
        if bytes == 0 {
            break;
        }
        let line = buffer
            .strip_suffix("\n")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap();
        lines.push(line);
    }
    print!("How many numbers to find?: ");
    stdout().flush().unwrap();

    let n: String = read!();
    let n = n.parse::<usize>().unwrap();

    println!("Processing...");

    let mut result: Option<(Vec<&u32>, u32)> = None;
    for perm in lines.iter().permutations(n) {
        let sum = perm.iter().fold(0, |acc, &v| acc + *v);
        if sum == 2020 {
            let mult = perm.iter().fold(1, |acc, &v| acc * *v);
            result = Some((perm, mult));
            break;
        }
    }
    let result = result;
    match result {
        Some((perm, solution)) => println!("Solution:  {:?} = {}", perm, solution),
        None => eprintln!("No solution found!"),
    }
}
