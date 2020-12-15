use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct BagQuantity(String, u8);

pub type BagRules = HashMap<String, HashSet<BagQuantity>>;

pub fn process_rules(s: String) -> BagRules {
    let mut generation_map: BagRules = HashMap::with_capacity(s.lines().count());

    for line in s.lines() {
        let left_colour = line.split_whitespace()
            // Colours are always the first two words
            .take(2)
            .collect::<String>();

        let remaining: String = line.split_whitespace()
            // Gets us to the start of the list of contents
            .skip(4)
            .collect::<Vec<&str>>()
            .join(" ");

        if remaining == "no other bags." { continue; }

        for rule in remaining.split(", ") {
            let number = rule.split_whitespace()
                .next()
                .expect("No number of bags")
                .parse::<u8>()
                // Ensured to be not "no" already
                .expect("Unable to parse number");

            let right_colour = rule.split_whitespace()
                // Skip the number
                .skip(1)
                // Take the next two words - always a colour
                .take(2)
                .collect::<String>();

            let bq = BagQuantity(right_colour, number);
            match generation_map.get_mut(&left_colour) {
                Some(set) => { set.insert(bq); },
                None => {
                    let mut set = HashSet::new();
                    set.insert(bq);
                    generation_map.insert(
                        left_colour.clone(),
                        set,
                    );
                },
            }
        }
    }
    //println!("{:#?}", generation_map);
    generation_map
}

fn remove_whitespace(s: &String) -> String {
    let mut out = s.clone();
    out.retain(|c| !c.is_whitespace());
    out
}

pub fn how_many_bags_can_contain(generation_map: &BagRules, search_term: &String) -> usize {
    // remove whitespace from input
    let search_term = remove_whitespace(search_term);

    generation_map.keys()
        .filter(|s| { **s != search_term })
        .map(|colour| {
            let foo = can_contain(generation_map, &search_term, colour);
            //println!();
            foo as usize
        })
        .sum::<usize>()
}

fn can_contain(generation_map: &BagRules, target_colour: &String, current_colour: &String) -> bool {
    if current_colour == target_colour {
        true
    } else {
        match generation_map.get(current_colour) {
            Some(set) => {
                // Quick check because we don't care about deeper solutions so long as we have found one
                let a = set.iter()
                    .any(|BagQuantity(colour, _)| { colour == target_colour });
                // Use closure for laziness but variable for readability
                let b = || set.iter()
                    .any(|BagQuantity(colour, _)| {
                        can_contain(generation_map, target_colour, colour)
                    });
                a || b()
            },
            None => {
                //print!(" nothing");
                false
            },
        }
    }
}

pub fn how_many_bags_within(generation_map: &BagRules, start_colour: &String) -> u64 {
    let start_colour = remove_whitespace(start_colour);

    if let Some(bag_quantities) = generation_map.get(&start_colour) {
        bag_quantities.iter()
            .map(|BagQuantity(colour, number)| {
                // The constant 1 being the current bag we're about to recurse into
                (*number as u64) * (1 + how_many_bags_within(generation_map, colour))
            }).sum()
    } else {
        0
    }
}
