use anyhow::{Result, Context};
use std::fs;
use day_9::{Buffer, find_contiguous_slice_summing_to};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_9/input")?;
    let mut buf = Buffer::new(&25);
    let mut input_used = Vec::new();
    let mut part_one = None;
    for n in input.lines().filter_map(|l| { l.parse::<u32>().ok() }) {
        if let Err(_) = buf.checked_insert(n) {
            part_one = Some(n);
            break;
        }
        input_used.push(n);
    }
    let part_one = part_one.context("No solution found")?;

    println!("First number to not comply with XMAS protocol is {}", part_one);

    let slice = find_contiguous_slice_summing_to(&input_used, &part_one).context("No solution for part two")?;
    let part_two = slice.iter().min().unwrap() + slice.iter().max().unwrap();

    println!("Encryption weakness: {}", part_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use day_9::{Buffer, find_contiguous_slice_summing_to};

    const INPUT: &str = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576\n";

    #[test]
    fn example_data_part_one() {
        let mut buf = Buffer::new(&5);
        for n in INPUT.lines().filter_map(|l| { l.parse::<u32>().ok() }) {
            if let Err(_) = buf.checked_insert(n) {
                assert_eq!(n, 127);
                break;
            }
        }
    }

    #[test]
    fn example_data_part_two() {
        let mut buf = Buffer::new(&5);
        let mut input_used = Vec::new();
        let mut part_one = None;
        for n in INPUT.lines().filter_map(|l| { l.parse::<u32>().ok() }) {
            if let Err(_) = buf.checked_insert(n) {
                part_one = Some(n);
                break;
            }
            input_used.push(n);
        }
        let part_one = part_one.expect("No solution found");

        println!("First number to not comply with XMAS protocol is {}", part_one);

        let slice = find_contiguous_slice_summing_to(&input_used, &part_one).expect("No solution for part two");
        let part_two = slice.iter().min().unwrap() + slice.iter().max().unwrap();

        assert_eq!(part_two, 62)
    }
}
