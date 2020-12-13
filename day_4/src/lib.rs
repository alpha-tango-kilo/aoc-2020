use anyhow::{Result, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

pub const REQUIRED_KEYS_TASK_ONE: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

pub const EYE_COLOURS: [&str; 7] = [
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth",
];

lazy_static! {
    static ref YEAR_RE: Regex = Regex::new("^[0-9]{4}$").unwrap();
    static ref HEIGHT_RE: Regex = Regex::new("^([5-7][0-9]in)|(1[5-9][0-9]cm)$").unwrap();
    static ref HEX_COLOUR_RE: Regex = Regex::new("^#([0-9]|[a-f]){6}$").unwrap();
}

pub fn process_batch_file(s: &str) -> Vec<Passport> {
    s.split("\n\n")
        .filter_map(|passport_string| {
            match Passport::new(passport_string) {
                Ok(p) => Some(p),
                Err(why) => { eprintln!("Dropped\n{:?}\n{}", passport_string, why); None },
            }
        })
        .collect()
}

pub fn count_valid_task_one(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|p| { p.validate_task_one() })
        .count()
}

pub fn count_valid_task_two(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|p| { p.validate_task_two() })
        .count()
}

#[derive(Debug)]
pub struct Passport {
    map: HashMap<String, String>,
}

impl Passport {
    pub fn new(s: &str) -> Result<Self> {
        Passport::from_str(s)
    }

    pub fn validate_task_one(&self) -> bool {
        REQUIRED_KEYS_TASK_ONE.iter()
            .all(|key| { self.map.contains_key(*key) })
    }

    pub fn validate_task_two(&self) -> bool {
        if !self.validate_task_one() { return false; }

        self.map.iter()
            .all(|(key, val)| -> bool {
                match key.as_str() {
                    "byr" => Passport::year_test(val.as_str(), 1920, 2002),
                    "iyr" => Passport::year_test(val.as_str(), 2010, 2020),
                    "eyr" => Passport::year_test(val.as_str(), 2020, 2030),
                    "hgt" => {
                        if HEIGHT_RE.is_match(val) {
                            if let Ok(height) = &val[..val.len()-2].parse::<u8>() {
                                if &val[val.len()-2..] == "cm" {
                                    *height >= 150 && *height <= 193
                                } else if &val[val.len()-2..] == "in" {
                                    *height >= 59  && *height <= 76
                                } else { false }
                            } else { false }
                        } else { false }
                    },
                    "hcl" => HEX_COLOUR_RE.is_match(val),
                    "ecl" => EYE_COLOURS.iter().any(|c| { c == val }),
                    "pid" => val.len() == 9 && val.parse::<u32>().is_ok(),
                    "cid" => true,
                    _     => false,
                }
            })
    }

    fn year_test(val: &str, min: u16, max: u16) -> bool {
        if YEAR_RE.is_match(val) {
            if let Ok(year) = val.parse::<u16>() {
                year >= min && year <= max
            } else { false }
        } else { false }
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = HashMap::with_capacity(8);

        for key_val in s.split_whitespace() {
            let mut parts = key_val.split(':');
            let key = parts.next()
                .context("Expected key")?;
            let val = parts.next()
                .context("Expected value")?;
            map.insert(key.to_string(), val.to_string());
        }

        Ok(Passport { map })
    }
}
