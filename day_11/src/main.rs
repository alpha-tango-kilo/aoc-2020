use std::fs;
use std::str::FromStr;

use anyhow::Result;

use day_11::{Area, Method};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_11/input")?;
    let mut area_one = Area::from_str(&input)?;
    let mut area_two = area_one.clone();

    area_one.iterate_until_stable(Method::NextTo);
    println!("Chairs occupied once stable: {}", area_one.occupied());

    area_two.iterate_until_stable(Method::LineOfSight);
    println!("Chairs occupied once stable: {}", area_two.occupied());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;

    use day_11::{Area, Method};

    const INPUT: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";

    #[test]
    fn example_data_part_two() -> Result<()> {
        let mut area = Area::from_str(INPUT)?;
        area.iterate_until_stable(Method::LineOfSight);
        assert_eq!(area.occupied(), 26);
        Ok(())
    }
}
