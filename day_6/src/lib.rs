pub mod part_one {
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
}

pub mod part_two {
    use anyhow::{Result, Context};
    use std::collections::HashSet;

    pub fn process_batch(s: String) -> Result<usize> {
        Ok(
            s.split("\n\n")
                .map(process_group)
                .collect::<Result<Vec<usize>>>()
                .context("Error parsing a group")?
                .iter()
                .sum::<usize>()
        )
    }

    fn process_group(s: &str) -> Result<usize> {
        let mut current = HashSet::new();
        let mut previous = HashSet::new();

        let start = s.lines().next().context("No first line in group")?;

        previous.extend(start.chars());

        //println!("Start! Previous: {:?}", previous);
        for person in s.lines().skip(1) {
            //println!("Checking:\t{}", person);
            person.chars()
                .for_each(|c| {
                    if previous.contains(&c) {
                        &current.insert(c.clone());
                    }
                });

            //println!("Current:\t{:?}\nPrevious:\t{:?}", current, previous);

            previous.clear();
            previous.extend(current.iter());
            current.clear();

            if previous.len() == 0 { break; }
        }
        //println!("Group complete. Result: {:?} (length {})\n", previous, previous.len());
        Ok(previous.len())
    }
}
