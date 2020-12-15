use anyhow::{Result, Context};
use std::fs;
use day_8::{Computer, read_program, generate_permutations};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_8/input")?;
    let program = read_program(input)?;
    let mut computer_one = Computer::new(program.clone());
    let part_one = computer_one.part_one_run()?;

    println!("Accumulator's value before first duplicate instruction: {}", part_one);

    let possible_programs = generate_permutations(program);
    let mut part_two = None;
    for program in possible_programs {
        if let Ok(n) = Computer::new(program).part_two_run() {
            part_two = Some(n);
            break;
        }
    }
    let part_two = part_two.context("No solution found for part two")?;

    println!("Accumulator's value after running fixed program: {}", part_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use day_8::{read_program, generate_permutations, Computer};

    const INPUT: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n";

    #[test]
    fn example_data_part_two() {
        let program = read_program(String::from(INPUT)).unwrap();
        let possible_programs = generate_permutations(program);
        let mut part_two = None;
        for program in possible_programs {
            if let Ok(n) = Computer::new(program).part_two_run() {
                part_two = Some(n);
                break;
            }
        }
        let part_two = part_two.expect("No solution found for part two");

        assert_eq!(part_two, 8);
    }
}
