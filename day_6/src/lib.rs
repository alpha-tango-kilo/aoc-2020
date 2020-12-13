use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

pub fn process_batch(s: String) -> Result<Vec<Group>> {
    s.split("\n\n")
        .map(Group::from_str)
        .collect()
}

#[derive(Debug)]
pub struct Group {
    pub questions_answered: HashSet<char>,
    pub people: usize,
}

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let people = s.lines().count();
        let mut questions_answered = HashSet::new();

        s.lines()
            .for_each(|line| {
                line.chars()
                    .for_each(|c| { questions_answered.insert(c); })
            });

        Ok(Group {
            questions_answered,
            people,
        })
    }
}
