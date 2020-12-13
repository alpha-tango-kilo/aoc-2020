use anyhow::Result;
use std::fs;
use day_4::{process_batch_file, count_valid};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_4/input")?;
    let passports = process_batch_file(&input);
    println!("Valid passports: {}", count_valid(&passports));
    Ok(())
}

#[cfg(test)]
mod tests {
    use day_4::{process_batch_file, count_valid};

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn example_data_part_one() {
        let passports = process_batch_file(INPUT);
        println!("Number of valid passports: {}", count_valid(&passports));
    }
}
