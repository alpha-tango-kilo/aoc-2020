use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    // Get input
    let input = fs::read_to_string("./day_1/input")?;
    let input = input.split('\n')
        .into_iter()
        .filter_map(|s| -> Option<u32> { s.parse().ok() })
        .collect::<Vec<u32>>();

    if let Some(n) = get_prod_of_those_that_sum(&2020, &input) {
        println!("Answer: {}", n);
    } else {
        println!("No answer found!");
    }

    Ok(())
}

fn get_prod_of_those_that_sum(to: &u32, list: &Vec<u32>) -> Option<u32> {
    for (n, a) in list.iter().enumerate() {
        let (_, bs) = list.split_at(n);
        if let Some((a, b)) = check_for_sum(to, a, bs) {
            return Some(a * b);
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
    use crate::get_prod_of_those_that_sum;

    #[test]
    fn example_data() {
        assert_eq!(
        get_prod_of_those_that_sum(&2020, &vec![1721, 979, 366, 299, 675, 1456]),
        Some(514579),
        );
    }
}
