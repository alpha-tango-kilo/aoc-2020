use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    // Get input
    let input = fs::read_to_string("./day_1/input")?;
    let input = input.split('\n')
        .into_iter()
        .filter_map(|s| -> Option<u32> { s.parse().ok() })
        .collect::<Vec<u32>>()
        .into_boxed_slice();

    if let Some((a, b)) = get_two_that_sum(&2020, &input) {
        println!("Product of two numbers that sum to 2020: {}", a * b);
    } else {
        println!("No answer found for two numbers that sum to 2020!");
    }

    if let Some((a, b, c)) = get_three_that_sum(&2020, &input) {
        println!("Product of three numbers that sum to 2020: {}", a * b * c);
    } else {
        println!("No answer found for three numbers that sum to 2020!");
    }

    Ok(())
}

fn get_two_that_sum<'a>(to: &u32, list: &'a [u32]) -> Option<(&'a u32, &'a u32)> {
    for (n, a) in list.iter().enumerate() {
        let (_, bs) = list.split_at(n);
        if let Some((a, b)) = check_for_sum(to, a, bs) {
            return Some((a, b));
        }
    }
    None
}

fn get_three_that_sum<'a>(to: &u32, list: &'a [u32]) -> Option<(&'a u32, &'a u32, &'a u32)> {
    for (n, a) in list.iter().enumerate() {
        let (_, bs) = list.split_at(n);
        if let Some((b, c)) = get_two_that_sum(&(to - a), bs) {
            return Some((a, b, c));
        }
    }
    None
}

fn check_for_sum<'a>(to: &u32, a: &'a u32, bs: &'a [u32]) -> Option<(&'a u32, &'a u32)> {
    for b in bs {
        if *a + *b == *to {
            return Some((a, b));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_data_part_one() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = get_two_that_sum(&2020, &input)
            .expect("No result!");

        assert_eq!(
            a * b,
            514579,
        );
    }

    #[test]
    fn example_data_part_two() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b, c) = get_three_that_sum(&2020, &input)
            .expect("No result!");

        assert_eq!(
            a * b * c,
            241861950,
        );
    }
}
