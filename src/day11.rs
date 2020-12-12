use crate::day11::Seat::{Empty, Floor, Occupied};
use std::fmt::{Display, Formatter, Result};
use std::fs::read_to_string;
use std::iter::repeat;
use std::ops::Add;

pub(crate) fn main() {
    let mut seats = Vec::new();
    for line in read_to_string("resources/day11.input").unwrap().lines() {
        let row: Vec<_> = line
            .chars()
            .map(|seat| match seat {
                'L' => Empty,
                '#' => Occupied,
                '.' => Floor,
                other => unimplemented!("Not a seat: {}", other),
            })
            .collect();
        seats.push(row);
    }

    let col_size = seats[0].len();
    let row_size = seats.len();

    let result_1 = part_1(&seats, None, row_size, col_size, 0);
    println!("Part 1 result: {}", result_1);
    let result_2 = part_2(&seats, None, row_size, col_size, 0);
    println!("Part 2 result: {}", result_2);
}

fn part_1(
    seats: &Vec<Vec<Seat>>,
    previous: Option<&Vec<Vec<Seat>>>,
    row_size: usize,
    col_size: usize,
    run_n: usize,
) -> u32 {
    let mut new_seats = mk_new_seats(row_size, col_size);
    for row in 0..row_size {
        for col in 0..col_size {
            let seat = seats[row][col];
            let count = adjacency(&seats, row_size, col_size, row, col).iter().fold(
                SeatCount::default(),
                |total, count| SeatCount {
                    occupied: total.occupied + count.occupied,
                    empty: total.empty + count.empty,
                },
            );

            if seat == Empty && count.occupied == 0 {
                new_seats[row][col] = Occupied
            } else if seat == Occupied && count.occupied >= 4 {
                new_seats[row][col] = Empty
            } else {
                new_seats[row][col] = seat
            };
        }
    }

    match previous {
        Some(prev) if (*prev == new_seats) => {
            let mut count = 0;
            for row in 0..row_size {
                for col in 0..col_size {
                    if new_seats[row][col] == Occupied {
                        count += 1
                    };
                }
            }
            count
        }
        _ => part_1(&new_seats, Some(seats), row_size, col_size, run_n + 1),
    }
}

fn adjacency(
    seats: &Vec<Vec<Seat>>,
    row_size: usize,
    col_size: usize,
    row: usize,
    col: usize,
) -> Vec<SeatCount> {
    let mut result = Vec::with_capacity(8);
    if row > 0 {
        if col > 0 {
            result.push(seats[row - 1][col - 1].seat_count())
        };
        result.push(seats[row - 1][col].seat_count());
        if col < col_size - 1 {
            result.push(seats[row - 1][col + 1].seat_count())
        };
    }
    if col > 0 {
        result.push(seats[row][col - 1].seat_count())
    };
    if col < col_size - 1 {
        result.push(seats[row][col + 1].seat_count())
    };
    if row < row_size - 1 {
        if col > 0 {
            result.push(seats[row + 1][col - 1].seat_count())
        };
        result.push(seats[row + 1][col].seat_count());
        if col < col_size - 1 {
            result.push(seats[row + 1][col + 1].seat_count())
        };
    }
    result
}

fn direction<A: Iterator<Item = usize>, B: Iterator<Item = usize>>(
    seats: &Vec<Vec<Seat>>,
    row_range: A,
    col_range: B,
) -> SeatCount {
    Iterator::zip(row_range, col_range)
        .map(|(r, c)| seats[r][c])
        .find(|s| *s != Floor)
        .map(|s| s.seat_count())
        .unwrap_or_default()
}

fn visible_seat_count(
    seats: &Vec<Vec<Seat>>,
    row: usize,
    row_size: usize,
    col: usize,
    col_size: usize,
) -> SeatCount {
    let up_left = direction(seats, (0..row).rev(), (0..col).rev());
    let up = direction(seats, (0..row).rev(), repeat(col));
    let up_right = direction(seats, (0..row).rev(), col + 1..col_size);
    let left = direction(seats, repeat(row), (0..col).rev());
    let down_left = direction(seats, row + 1..row_size, (0..col).rev());
    let down = direction(seats, row + 1..row_size, repeat(col));
    let down_right = direction(seats, row + 1..row_size, col + 1..col_size);
    let right = direction(seats, repeat(row), col + 1..col_size);

    up_left + up + up_right + left + right + down_left + down + down_right
}

#[cfg(test)]
mod tests {
    use crate::day11::visible_seat_count;
    use crate::day11::Seat::{Empty, Floor, Occupied};

    #[test]
    fn visible_count() {
        assert_eq!(
            visible_seat_count(
                &vec![
                    vec![Empty, Empty, Empty],
                    vec![Empty, Empty, Empty],
                    vec![Empty, Empty, Empty]
                ],
                1,
                3,
                1,
                3
            )
            .empty,
            8
        );

        assert_eq!(
            visible_seat_count(
                &vec![
                    vec![Occupied, Floor, Occupied],
                    vec![Occupied, Occupied, Occupied],
                    vec![Occupied, Floor, Occupied]
                ],
                1,
                3,
                0,
                3
            )
            .occupied,
            3
        );
    }
}

fn part_2(
    seats: &Vec<Vec<Seat>>,
    previous: Option<&Vec<Vec<Seat>>>,
    row_size: usize,
    col_size: usize,
    run_n: usize,
) -> u32 {
    let mut new_seats = mk_new_seats(row_size, col_size);
    for row in 0..row_size {
        for col in 0..col_size {
            let count = visible_seat_count(&seats, row, row_size, col, col_size);
            match seats[row][col] {
                Occupied if count.occupied >= 5 => new_seats[row][col] = Empty,
                Empty if count.occupied == 0 => new_seats[row][col] = Occupied,
                other => new_seats[row][col] = other,
            };
        }
    }

    match previous {
        Some(prev) if (*prev == new_seats) => {
            let mut count = 0;
            for row in 0..row_size {
                for col in 0..col_size {
                    if new_seats[row][col] == Occupied {
                        count += 1
                    };
                }
            }
            count
        }
        _ => part_2(&new_seats, Some(seats), row_size, col_size, run_n + 1),
    }
}

fn mk_new_seats(row_size: usize, col_size: usize) -> Vec<Vec<Seat>> {
    vec![vec![Empty; col_size]; row_size]
}

fn _print_seats(seats: &Vec<Vec<Seat>>, row_size: usize, col_size: usize) -> () {
    for row in 0..row_size {
        for col in 0..col_size {
            print!("{}", seats[row][col]);
        }
        println!();
    }
}

trait Seats {
    fn seat_count(self: Self) -> SeatCount;
}

impl Seats for Seat {
    fn seat_count(self: Self) -> SeatCount {
        match self {
            Empty => SeatCount {
                empty: 1,
                occupied: 0,
            },
            Occupied => SeatCount {
                empty: 0,
                occupied: 1,
            },
            Floor => SeatCount {
                empty: 0,
                occupied: 0,
            },
        }
    }
}

#[derive(Default, Debug)]
struct SeatCount {
    occupied: u32,
    empty: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Empty => write!(f, "L"),
            Occupied => write!(f, "#"),
            Floor => write!(f, "."),
        }
    }
}

impl Add for SeatCount {
    type Output = SeatCount;

    fn add(self, rhs: SeatCount) -> SeatCount {
        SeatCount {
            occupied: self.occupied + rhs.occupied,
            empty: self.empty + rhs.empty,
        }
    }
}
