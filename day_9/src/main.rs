use anyhow::Result;
use std::fs;
use day_9::Buffer;

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_9/input")?;
    let mut buf = Buffer::new(&25);
    for n in input.lines().filter_map(|l| { l.parse::<u32>().ok() }) {
        if let Err(_) = buf.checked_insert(n) {
            println!("First number to not comply with XMAS protocol is {}", n);
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use day_9::Buffer;

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
}
