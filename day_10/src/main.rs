use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use day_10::load_adapters_from_string;

    const INPUT: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
    #[test]
    fn example_data_part_one() {
        let hell = load_adapters_from_string(INPUT);
        println!("{:#?}", hell);
        assert_eq!(0, 0);
    }
}
