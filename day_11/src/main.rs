use std::fs;
use std::str::FromStr;

use anyhow::Result;

use day_11::Area;

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_11/input")?;
    let mut area = Area::from_str(&input)?;
    area.iterate_until_stable();
    println!("Chairs occupied once stable: {}", area.occupied());
    Ok(())
}
