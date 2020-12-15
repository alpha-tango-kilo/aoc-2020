use anyhow::Result;
use std::fs;
use day_7::{process_rules, how_many_bags_can_contain, how_many_bags_within};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_7/input")?;
    let rules = process_rules(input);
    let part_one = how_many_bags_can_contain(&rules, &String::from("shiny gold"));

    println!("{} bags can contain a shiny gold bag", part_one);

    let part_two = how_many_bags_within(&rules, &String::from("shiny gold"));

    println!(
        "There are {} bag{} within a single shiny gold bag",
        part_two,
        if part_two == 1 { "" } else { "s" }
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use day_7::{process_rules, how_many_bags_can_contain};

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const INPUT_TWO: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn example_data_part_one() {
        let rules = process_rules(INPUT.to_string());
        let answer = how_many_bags_can_contain(&rules, &String::from("shiny gold"));
        assert_eq!(answer, 4);
    }

    #[test]
    fn example_data_part_two() {
        let rules = process_rules(INPUT.to_string());
    }
}
