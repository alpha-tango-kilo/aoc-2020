use anyhow::Result;
use std::fs;
use day_10::{load_adapters_from_string, part_one_fast};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_10/input")?;
    let adapters = load_adapters_from_string(&input)?;
    let part_one = part_one_fast(&adapters);
    println!("The product of the number of one jolt and three jolt differences is {}", part_one);
    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use day_10::{load_adapters_from_string, part_one_fast};

    const INPUT: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
    #[test]
    fn example_data_part_one() -> Result<()> {
        let hell = load_adapters_from_string(INPUT)?;
        let answer = part_one_fast(&hell);
        assert_eq!(answer, 35);
        Ok(())
    }
}
