use std::collections::{HashMap, HashSet};

pub fn process_rules(s: String) -> HashMap<String, HashSet<String>> {
    let mut generation_map: HashMap<String, HashSet<String>> = HashMap::with_capacity(500);

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
            let right_colour = rule.split_whitespace()
                // Skip the number
                .skip(1)
                // Take the next two words - always a colour
                .take(2)
                .collect::<String>();

            match generation_map.get_mut(&left_colour) {
                Some(set) => { set.insert(right_colour); },
                None => {
                    let mut set = HashSet::new();
                    set.insert(right_colour);
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

pub fn how_many_bags_can_contain(generation_map: &HashMap<String, HashSet<String>>, search_term: &String) -> usize {
    // remove whitespace from input
    let mut search_term = search_term.clone();
    search_term.retain(|c| !c.is_whitespace());

    generation_map.keys()
        .filter(|s| { **s != search_term })
        .map(|colour| {
            let foo = can_contain(generation_map, &search_term, colour);
            //println!();
            foo as usize
        })
        .sum::<usize>()
}

fn can_contain(generation_map: &HashMap<String, HashSet<String>>, target_colour: &String, current_colour: &String) -> bool {
    if current_colour == target_colour {
        //print!(" {}!", search_term);
        true
    } else {
        //print!(" {},", search_in);
        match generation_map.get(current_colour) {
            Some(set) => {
                if set.contains(target_colour) {
                    // Fast return because we don't care about deeper solutions so long as we have found one
                    //print!(" {}!", search_term);
                    true
                } else {
                    set.iter()
                        .any(|colour| {
                            //print!(" {{");
                            let foo = can_contain(generation_map, target_colour, colour);
                            //print!(" }},");
                            foo
                        })
                }
            },
            None => {
                //print!(" nothing");
                false
            },
        }
    }
}
