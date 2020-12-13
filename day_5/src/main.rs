use anyhow::Result;
use std::fs;
use day_5::{parse_lines, Seat};
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_5/input")?;
    let seats = parse_lines(&input, &128, &8)?;
    // Unwrap is guaranteed as Seat impl Ord
    let highest_id = seats.iter().max().unwrap();
    println!("Highest ID: {}", highest_id.get_id());

    let seats = seats.into_iter()
        .filter(|s| { s.row != 0 && s.row != 128})
        .map(|s| { s.get_id() })
        .collect::<Vec<_>>();

    for (n, a_id) in seats.iter().enumerate() {
        let (_, after) = seats.split_at(n);
        for b_id in after {
            let middle = if a_id > b_id { a_id + 1 } else { b_id + 1 };
            if !seats.contains(&middle) {
                println!("Sit in seat {}", middle);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use day_5::parse_lines;

    const INPUT: &str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n";
    const RESULTS: [(u8, u8, u16); 3] = [
        ( 70, 7, 567),
        ( 14, 7, 119),
        (102, 4, 820),
    ];

    #[test]
    fn example_data_part_one() {
        parse_lines(&INPUT.to_string(), &128, &8)
            .expect("Failed to parse input")
            .iter()
            .zip(RESULTS.iter())
            .for_each(|(seat, (row, column, id))| {
                assert_eq!(
                    seat.row,
                    *row,
                    "Rows incorrect",
                );
                assert_eq!(
                    seat.column,
                    *column,
                    "Columns incorrect"
                );
                assert_eq!(
                    seat.get_id(),
                    *id,
                    "ID incorrect"
                );
            });
    }
}
