use anyhow::{Result, Context};
use std::str::FromStr;
use std::collections::HashMap;

pub const REQUIRED_KEYS_TASK_ONE: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "cid",
];

pub fn process_batch_file(s: &str) -> Vec<Passport> {
    s.split("\n\n")
        .filter_map(|passport_string| { Passport::new(passport_string).ok() })
        .collect()
}

pub fn count_valid(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|p| { p.validate_task_one() })
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
