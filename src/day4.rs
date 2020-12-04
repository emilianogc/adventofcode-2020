use std::io::stdin;
use regex::Regex;
use lazy_static::lazy_static;

pub(crate) fn main() {
    let mut ok = 0;
    let file = std::fs::read_to_string("src/day4.input").unwrap();
    let lines: Vec<&str> = file.split("\n\n").collect();
    for line in lines {
        let flags = line
            .split(|c| c == ' ' || c == '\n')
            .flat_map(|v| v.split_once(':').into_iter())
            .map(|prev| match prev {
                ("byr", v) if birth_year(v) => 0x10000000,
                ("iyr", v) if issue_year(v) => 0x01000000,
                ("eyr", v) if expiration_year(v) => 0x00100000,
                ("hgt", v) if height(v) => 0x00010000,
                ("hcl", v) if hair_color(v) => 0x00001000,
                ("ecl", v) if eye_color(v) => 0x00000100,
                ("pid", v) if passport_id(v) => 0x00000010,
                ("cid", _) => 0x00000001,
                _ => 0,
            })
            .fold(0, |flags, flag| flags | flag);

        if flags & 0x11111111 == 0x11111111 { ok += 1}
        else if flags & 0x11111110 == 0x11111110 { ok += 1 };
    }

    println!("Ok passports {}", ok)
}


fn birth_year(value: &str) -> bool {
    valid_year(value, 1920, 2002)
}

fn issue_year(value: &str) -> bool {
    valid_year(value, 2010, 2020)
}

fn expiration_year(value: &str) -> bool {
    valid_year(value, 2020, 2030)
}

fn height(value: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("(\\d+)(cm|in)").unwrap();
    }
    let capture = REGEX.captures(value);
    match capture {
        Some(cap) if &cap[2] == "cm" => cap[1].parse::<u16>().map(|v| v >= 150 && v <= 193).unwrap_or(false),
        Some(cap) if &cap[2] == "in" => cap[1].parse::<u16>().map(|v| v >= 59 && v <= 76).unwrap_or(false),
        _ => false
    }
}

fn hair_color(value: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
    }
    REGEX.is_match(value)
}

fn eye_color(value: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
    }
    REGEX.is_match(value)
}

fn passport_id(value: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    REGEX.is_match(value)
}

fn valid_year(value: &str, min: u16, max: u16) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("\\d{4}").unwrap();
    }
    REGEX.is_match(value) && value.parse::<u16>().map(|year| year >= min && year <= max).unwrap_or(false)
}