use anyhow::{Result, Context};
use std::fs;
use day_6::{part_one, part_two};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_6/input")?;
    let groups = part_one::process_batch(input.clone())
        .context("Failed to parse data")?;

    let all_questions_answered = groups.iter()
        .map(|group| { group.questions_answered.len() })
        .sum::<usize>();

    println!("Total number of questions answered: {}", all_questions_answered);

    let count = part_two::process_batch(input)
        .context("Failed to parse data")?;

    println!("Part two answer: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use day_6::{part_one, part_two};

    const INPUT: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn example_data_part_one() {
        assert_eq!(
            part_one::process_batch(INPUT.to_string())
                .expect("Failed to parse data")
                .iter()
                .map(|group| { group.questions_answered.len() })
                .sum::<usize>(),
            11,
        );
    }

    #[test]
    fn example_data_part_two() {
        assert_eq!(
            part_two::process_batch(INPUT.to_string())
                .expect("Failed to parse data"),
            6
        )
    }
}
