use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn example_data_part_one() {

    }
}
