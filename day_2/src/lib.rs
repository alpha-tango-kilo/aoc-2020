use std::{ops::BitXor, str::FromStr};
use regex::Regex;

pub trait Validatable {
    fn valid(&self) -> bool;
}

pub fn count_valid(entries: &Vec<impl Validatable>) -> usize {
    let mut valid = 0;
    entries.iter().for_each(|e| { if e.valid() { valid += 1 } });
    valid
}

pub fn parse_lines<T: FromStr>(string: String) -> Vec<T> {
    string.lines().filter_map(|line| { T::from_str(line).ok() }).collect()
}

#[derive(Debug)]
pub struct Entry<T: FromStr> {
    rule: T,
    pwd: String,
}

impl<T: FromStr> FromStr for Entry<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");

        // Don't nest match statements for readability
        let rule = match parts.next() {
            Some(rule_string) => T::from_str(rule_string),
            None => return Err(format!("String in incorrect format : {:?}", s)),
        };
        let rule = match rule {
            Ok(r) => r,
            Err(_) => return Err(String::from("Failed to create rule")),
        };

        let pwd = match parts.next() {
            Some(str) => String::from(str),
            None => return Err(format!("No second half to input string: {:?}", s)),
        };

        Ok(Entry {
            rule,
            pwd,
        })
    }
}

impl Validatable for Entry<part_one::Rule> {
    fn valid(&self) -> bool {
        let re = Regex::new(&self.rule.character.to_string()).expect("Character formed bad regex");
        let occurences = re.find_iter(&self.pwd).count();

        occurences >= self.rule.min_occurences && occurences <= self.rule.max_occurences
    }
}

impl Validatable for Entry<part_two::Rule> {
    fn valid(&self) -> bool {
        let a = self.pwd.chars().nth(self.rule.indexes.0).unwrap() == self.rule.character;
        let b = self.pwd.chars().nth(self.rule.indexes.1).unwrap() == self.rule.character;

        a.bitxor(b)
    }
}

pub mod part_one {
    use std::str::FromStr;
    use regex::Regex;

    #[derive(Debug)]
    pub struct Rule {
        pub(crate) min_occurences: usize,
        pub(crate) max_occurences: usize,
        pub(crate) character: char,
    }

    impl FromStr for Rule {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new("[0-9]+").unwrap();
            let mut matches = re.find_iter(s);

            let min_occurences = match matches.next() {
                Some(m) => m,
                None => return Err(format!("Minimum occurences not found in {:?}", s)),
            };
            let min_occurences = match min_occurences.as_str().parse::<usize>() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            };

            let max_occurences = match matches.next() {
                Some(m) => m,
                None => return Err(format!("Maximum occurences not found in {:?}", s)),
            };
            let max_occurences = match max_occurences.as_str().parse::<usize>() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            };

            Ok(Rule {
                min_occurences,
                max_occurences,
                // Any error here would have already been thrown above
                character: s.chars().last().unwrap(),
            })
        }
    }
}

pub mod part_two {
    use std::str::FromStr;
    use regex::Regex;

    #[derive(Debug)]
    pub struct Rule {
        pub indexes: Indexes,
        pub(crate) character: char,
    }

    #[derive(Debug)]
    pub struct Indexes(pub(crate) usize, pub(crate) usize);

    impl FromStr for Rule {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new("[0-9]+").unwrap();
            let mut matches = re.find_iter(s);

            let index_one = match matches.next() {
                Some(m) => m,
                None => return Err(format!("Minimum occurences not found in {:?}", s)),
            };
            let index_one = match index_one.as_str().parse::<usize>() {
                Ok(n) => n - 1,
                Err(e) => return Err(e.to_string()),
            };

            let index_two = match matches.next() {
                Some(m) => m,
                None => return Err(format!("Maximum occurences not found in {:?}", s)),
            };
            let index_two = match index_two.as_str().parse::<usize>() {
                Ok(n) => n - 1,
                Err(e) => return Err(e.to_string()),
            };

            Ok(Rule {
                indexes: Indexes(index_one, index_two),
                // Any error here would have already been thrown above
                character: s.chars().last().unwrap(),
            })
        }
    }
}
