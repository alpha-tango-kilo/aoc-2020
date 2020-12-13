use anyhow::{Result, Context};
use std::fs;
use day_6::process_batch;

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_6/input")?;
    let groups = process_batch(input)
        .context("Failed to parse data")?;

    let all_questions_answered = groups.iter()
        .map(|group| { group.questions_answered.len() })
        .sum::<usize>();

    println!("All questions answered: {}", all_questions_answered);
    Ok(())
}

#[cfg(test)]
mod tests {
    use day_6::process_batch;

    const INPUT: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn example_data_part_one() {
        assert_eq!(
            process_batch(INPUT.to_string())
                .expect("Failed to parse data")
                .iter()
                .map(|group| { group.questions_answered.len() })
                .sum::<usize>(),
            11,
        );
    }
}
