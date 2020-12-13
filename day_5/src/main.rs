use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n";
    const RESULTS: [(u8, u8, u16); 3] = [
        ( 70, 7, 567),
        ( 14, 7, 119),
        (102, 4, 820),
    ];

    #[test]
    fn example_data_part_one() {

    }
}
