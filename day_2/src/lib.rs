pub mod part_one {
    use std::str::FromStr;
    use regex::Regex;

    #[derive(Debug)]
    pub struct Entry {
        rule: Rule,
        pwd: String,
    }

    impl FromStr for Entry {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(": ");

            // Don't nest match statements for readability
            let rule = match parts.next() {
                Some(rule_string) => Rule::from_str(rule_string),
                None => return Err(format!("String in incorrect format : {:?}", s)),
            };
            let rule = match rule {
                Ok(r) => r,
                Err(why) => return Err(why),
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

    impl Entry {
        pub fn valid(&self) -> bool {
            let re = Regex::new(&self.rule.character.to_string()).expect("Character formed bad regex");
            let occurences = re.find_iter(&self.pwd).count();

            occurences >= self.rule.min_occurences && occurences <= self.rule.max_occurences
        }
    }

    #[derive(Debug)]
    struct Rule {
        min_occurences: usize,
        max_occurences: usize,
        character: char,
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

    pub fn parse_lines(string: String) -> Vec<Entry> {
        string.lines().filter_map(|line| { Entry::from_str(line).ok() }).collect()
    }

    pub fn count_valid(entries: &Vec<Entry>) -> usize {
        let mut valid: usize = 0;
        entries.iter().for_each(|e| { if e.valid() { valid += 1} });
        valid
    }
}
