use anyhow::{Result, Context};
use std::fs;
use day_3::{Slope, Vec2D, trees_hit};

const PART_TWO_SLOPES: [Vec2D; 5] = [
    Vec2D { x: 1, y: 1},
    Vec2D { x: 3, y: 1},
    Vec2D { x: 5, y: 1},
    Vec2D { x: 7, y: 1},
    Vec2D { x: 1, y: 2},
];

fn main() -> Result<()> {
    let input = fs::read_to_string("./day_3/input")?;
    let slope = Slope::new(&input).context("Failed to parse slope")?;
    let start = Vec2D { x: 0, y: 0 };
    let vector = Vec2D { x: 3, y: 1 };

    let part_one_trees_hit = trees_hit(&slope, &start, &vector)
        .context("Failed to calculate trees hit")?;

    println!("Trees hit: {}", part_one_trees_hit);

    let part_two_product_of_all_trees_hit = PART_TWO_SLOPES.iter()
        .filter_map(|vector| { trees_hit(&slope, &start, vector).ok() })
        .product::<usize>();

    println!("Product of all trees hit: {}", part_two_product_of_all_trees_hit);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Slope, Vec2D, trees_hit, PART_TWO_SLOPES};

    const INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn example_data_part_one() {
        let slope = Slope::new(&INPUT).expect("Failed to parse slope");
        let start = Vec2D { x: 0, y: 0 };
        let vector = Vec2D { x: 3, y: 1 };

        let trees_hit = trees_hit(&slope, &start, &vector)
            .expect("Failed to calculate trees hit");
        assert_eq!(trees_hit, 7);
    }

    #[test]
    fn example_data_part_two() {
        let slope = Slope::new(&INPUT).expect("Failed to parse slope");
        let start = Vec2D { x: 0, y: 0 };

        let part_two_product_of_all_trees_hit = PART_TWO_SLOPES.iter()
            .filter_map(|vector| { trees_hit(&slope, &start, vector).ok() })
            .product::<usize>();

        assert_eq!(part_two_product_of_all_trees_hit, 336);
    }
}
