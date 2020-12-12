use std::{error::Error, fs};

use day_2::part_one::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./day_2/input")?;
    let entries = parse_lines(input);
    println!("Valid passwords: {}", count_valid(&entries));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_data_part_one() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let entries = parse_lines(input);
        let valid = count_valid(&entries);
        assert_eq!(valid, 2);
    }
}
