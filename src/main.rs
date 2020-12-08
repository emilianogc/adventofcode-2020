#![feature(str_split_once)]
#![feature(iterator_fold_self)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use colored::*;
use std::io::Write;
use std::{env, io};
use text_io::read;

fn main() {
    let arg = env::args().nth(1).and_then(|v| v.parse::<u8>().ok());
    let selection = match arg {
        Some(selection) => selection,
        _ => {
            println!();
            println!();
            println!(
                "1.                   . {}{}{} .                    ",
                "....".forest_green(),
                "|".bark_brown(),
                "....".forest_green()
            );
            println!("2.");
            println!("3.");

            print!("\nSelection: ");
            io::stdout().flush().unwrap();
            let read: String = read!();
            read.parse::<u8>().unwrap()
        }
    };

    match selection {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        7 => day7::main(),
        8 => day8::main(),
        _ => eprintln!("Not a valid option"),
    }
}

trait CustomColors {
    fn bark_brown(self) -> ColoredString
    where
        Self: Sized + Colorize,
    {
        self.truecolor(140, 88, 80)
    }

    fn forest_green(self) -> ColoredString
    where
        Self: Sized + Colorize,
    {
        self.truecolor(34, 139, 34)
    }
}

impl<'a> CustomColors for &'a str {}
