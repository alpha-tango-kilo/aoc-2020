use anyhow::{Result, Error};
use std::cmp::Ordering;

// Signature means whole reading fails if any line fails to parse
pub fn parse_lines(input: &String, rows: &u8, columns: &u8) -> Result<Vec<Seat>> {
    input.lines()
        .map(|line| { Seat::new(line, rows, columns) })
        .collect()
}

fn bsp(s: &str, lower: char, higher: char, max: &u8) -> Result<u8> {
    let mut low = 0;
    let mut high = *max;

    //println!("{}", s);
    for c in s.chars() {
        let adjustment = (high - low) / 2;
        //println!("High: {}\tLow: {}\tAdjustment:{}", high, low, adjustment);
        if c == lower {
            high -= adjustment;
        } else if c == higher {
            low += adjustment;
        } else {
            return Err(Error::msg("Bad letter in string"));
        }
    }
    //println!("High: {}\tLow: {}", high, low);

    Ok(low)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Seat {
    pub row: u8,
    pub column: u8,
}

impl Seat {
    pub fn new(s: &str, rows: &u8, columns: &u8) -> Result<Self> {
        // Step 0 - validation
        let row_iterations = (*rows as f32).log2();
        if row_iterations.fract() != 0f32 {
            return Err(Error::msg("Rows doesn't appear to be a power of two"))
        }

        if (*columns as f32).log2().fract() != 0f32 {
            return Err(Error::msg("Columns doesn't appear to be a power of two"))
        }

        // Rows
        //println!("No. of characters: {}", row_iterations);
        let row = bsp(
            &s[..(row_iterations as usize)],
            'F',
            'B',
            rows,
        )?;

        // Columns
        //println!("No. of characters: {}", s.len() - row_iterations as usize);
        let column = bsp(
            &s[(row_iterations as usize)..],
            'L',
            'R',
            columns,
        )?;

        Ok(Seat {
            row,
            column,
        })
    }

    pub fn get_id(&self) -> u16 {
        (self.row as u16) * 8 + (self.column as u16)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}
