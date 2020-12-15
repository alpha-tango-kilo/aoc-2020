use anyhow::Result;
use std::fs;
use day_8::{Computer, read_program};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_8/input")?;
    let instructions = read_program(input)?;
    let mut computer = Computer::new(instructions);
    let part_one = computer.debug_run()?;

    println!("Accumulator's value before first duplicate instruction: {}", part_one);

    Ok(())
}
