use std::io::stdin;

use text_io::scan;

pub(crate) fn main() {
    println!("Enter lines (Ctrl+D to end): ");
    let mut correct1 = 0;
    let mut correct2 = 0;
    loop {

        let mut buffer = String::new();
        let bytes = stdin().read_line(&mut buffer);

        match bytes {
            Ok(0) => break,
            Err(e) => eprintln!("Failed reading: {}", e),
            _ => {}
        }

        let (a, b, c, pass) : (usize, usize, char, String);
        scan!(buffer.bytes() => "{}-{} {}: {}", a, b, c, pass);

        let count = pass.chars().filter(|cc| c == *cc).count();
        if (count >= a) & (count <= b) {
            correct1 += 1
        }

        if (pass.chars().nth(a - 1).unwrap() == c)  ^ (pass.chars().nth(b - 1).unwrap() == c) {
            correct2 += 1
        }
    }
    println!("Amount of correct passwords (step 1): {}", correct1);
    println!("Amount of correct passwords (step 2): {}", correct2);
}