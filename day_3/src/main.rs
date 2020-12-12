use anyhow::{Result, Context};
use std::fs;
use day_3::{Slope, Vec2D, trees_hit};

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_3/input")?;
    let slope = Slope::new(&input).context("Failed to parse slope")?;
    let start = Vec2D { x: 0, y: 0 };
    let vector = Vec2D { x: 3, y: 1 };

    let trees_hit = trees_hit(&slope, &start, &vector)
        .context("Failed to calculate trees hit")?;

    println!("Trees hit: {}", trees_hit);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Slope, Vec2D, trees_hit};

    #[test]
    fn example_data_part_one() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

        let slope = Slope::new(input).expect("Failed to parse slope");
        let start = Vec2D { x: 0, y: 0 };
        let vector = Vec2D { x: 3, y: 1 };

        let trees_hit = trees_hit(&slope, &start, &vector)
            .expect("Failed to calculate trees hit");
        assert_eq!(trees_hit, 7);
    }
}
