use std::fs;
use std::str::FromStr;

use anyhow::Result;

use day_12::{Instruction, Ship};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_12/input")?;
    let mut ship = Ship::new();
    let instructions = input.lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;
    ship.apply_instructions(&instructions);

    println!("Manhatten distance from origin: {}", ship.manhatten_from_origin());
    Ok(())
}
