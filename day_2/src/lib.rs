use anyhow::{Context, Result};
use regex::Regex;
use std::{ops::BitXor, str::FromStr};

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
pub struct Entry<T: FromStr>  {
    rule: T,
    pwd: String,
}

impl<T: FromStr> FromStr for Entry<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(": ");
        let rule_string = parts.next()
            .context(format!("String in incorrect format : {:?}", s))?;
        // This is awful practice but I can't get the error out without the compiler throwing a hissy fit
        let rule = T::from_str(rule_string).ok()
            .context(format!("Failed to parse {} into a rule", rule_string))?;

        let pwd = parts.next()
            .context(format!("No second half to input string: {:?}", s))?
            .to_string();

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
    use anyhow::{Context, Result};
    use regex::Regex;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct Rule {
        pub(crate) min_occurences: usize,
        pub(crate) max_occurences: usize,
        pub(crate) character: char,
    }

    impl FromStr for Rule {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let re = Regex::new("[0-9]+").unwrap();
            let mut matches = re.find_iter(s);

            let min_occurences = matches.next()
                .context(format!("Minimum occurences not found in {:?}", s))?
                .as_str()
                .parse::<usize>()?;

            let max_occurences = matches.next()
                .context(format!("Maximum occurences not found in {:?}", s))?
                .as_str()
                .parse::<usize>()?;

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
    use anyhow::{Context, Result};

    #[derive(Debug)]
    pub struct Rule {
        pub indexes: Indexes,
        pub(crate) character: char,
    }

    #[derive(Debug)]
    pub struct Indexes(pub(crate) usize, pub(crate) usize);

    impl FromStr for Rule {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let re = Regex::new("[0-9]+").unwrap();
            let mut matches = re.find_iter(s);

            let index_one = matches.next()
                .context(format!("Index one not found in {:?}", s))?
                .as_str()
                .parse::<usize>()?
                - 1;

            let index_two = matches.next()
                .context(format!("Index two not found in {:?}", s))?
                .as_str()
                .parse::<usize>()?
                - 1;

            Ok(Rule {
                indexes: Indexes(index_one, index_two),
                // Any error here would have already been thrown above
                character: s.chars().last().unwrap(),
            })
        }
    }
}
