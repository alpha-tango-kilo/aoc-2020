use anyhow::{Result, Context};
use std::collections::HashMap;

pub fn process_rules(s: String) -> Result<HashMap<String, Vec<(String, usize)>>> {
    let mut generation_map: HashMap<String, Vec<(String, usize)>> = HashMap::with_capacity(500);

    for line in s.lines() {
        let left_colour = line.split_whitespace()
            // Colours are always the first two words
            .take(2)
            .collect::<String>();

        let remaining = line.split_whitespace()
            // Gets us to the start of the list of contents
            .skip(4)
            .collect::<Vec<&str>>()
            .join(" ");

        for rule in remaining.split(", ") {
            let number_str = rule.split_whitespace()
                // Number is always the first word
                .take(1)
                .collect::<String>();
            if number_str == "no" { break; }
            let number = number_str.parse::<usize>()
                .context(format!("This should be a number: {}", number_str))?;

            let right_colour = rule.split_whitespace()
                // Skip the number
                .skip(1)
                // Take the next two words - always a colour
                .take(2)
                .collect::<String>();

            match generation_map.get_mut(&left_colour) {
                Some(vec) => vec.push((right_colour, number)),
                None => {
                    generation_map.insert(
                        left_colour.clone(),
                        vec![(right_colour, number)]
                    );
                },
            }
        }
    }
    //println!("{:#?}", generation_map);
    Ok(generation_map)
}

pub fn how_many_bags_can_contain(generation_map: &HashMap<String, Vec<(String, usize)>>, s: &String) -> usize {
    // remove whitespace from input
    let mut search_term = s.clone();
    search_term.retain(|c| !c.is_whitespace());

    generation_map.keys()
        .map(|colour| { recurse_this(generation_map, colour, &search_term) })
        .sum::<usize>()
}

fn recurse_this(generation_map: &HashMap<String, Vec<(String, usize)>>, search_in: &String, search_term: &String) -> usize {
    if search_in == search_term {
        1
    } else {
        match generation_map.get(search_in) {
            Some(vec) => {
                vec.iter()
                    .map(|(s, _)| { recurse_this(generation_map, s, search_term)})
                    .sum::<usize>()
            },
            None => 0,
        }
    }
}
