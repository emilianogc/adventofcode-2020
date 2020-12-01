mod day1;

use colored::*;
use text_io::read;
use std::io;
use std::io::Write;

fn main() {
    println!();
    println!();
    println!("1.                   . {}{}{} .                    ", "....".forest_green(), "|".bark_brown(), "....".forest_green());
    println!("2.");
    println!();

    print!("\nSelection: ");
    io::stdout().flush().unwrap();

    let selection: String = read!();
    let selection = selection.parse::<i32>().unwrap();
    match selection {
        1 => day1::main(),
        _ => eprintln!("Not a valid option")
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
        self.truecolor(34,139,34)
    }

}

impl<'a> CustomColors for &'a str  {}
