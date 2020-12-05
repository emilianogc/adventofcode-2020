use bit_set::BitSet;
use std::fs::read_to_string;

pub(crate) fn main() {
    let file = read_to_string("src/day5.input").unwrap();
    let mut max_id = 0;
    let mut seats = vec![vec![false; 8]; 128];
    let mut ids = BitSet::new();

    for line in file.split('\n') {
        let mut front_row = 0;
        let mut back_row = 127;

        let mut max_column = 7;
        let mut min_column = 0;

        for char in line.chars() {
            match char {
                'F' => {
                    back_row -= (back_row - front_row + 1) / 2;
                }

                'B' => {
                    front_row += (back_row - front_row + 1) / 2;
                }
                'L' => {
                    max_column -= (max_column - min_column + 1) / 2;
                }
                'R' => {
                    min_column += (max_column - min_column + 1) / 2;
                }
                _ => continue,
            }
        }
        let id = front_row * 8 + max_column;
        if id > max_id {
            max_id = id
        };
        ids.insert(id);

        seats[front_row][max_column] = true;
    }

    for row in 0..128 {
        for column in 0..8 {
            let id = row * 8 + column;
            if !seats[row][column] && ids.contains(id + 1) && ids.contains(id - 1) {
                println!("Your seat: {}", id);
                break;
            }
        }
    }

    println!("Max id: {}", max_id);
}
