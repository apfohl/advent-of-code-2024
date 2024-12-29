use crate::common::lines;
use std::io::Error;

fn load_puzzle() -> Result<Vec<usize>, Error> {
    let puzzle = lines("inputs/day_11_test.txt")?
        .find_map(|line| match line {
            Ok(line) => Some(
                line.split_whitespace()
                    .filter_map(|number| number.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            ),
            Err(_) => None,
        })
        .unwrap_or_default();

    Ok(puzzle)
}

#[test]
fn part_one() -> Result<(), Error> {
    let stones = load_puzzle()?;

    println!("{:?}", stones);

    Ok(())
}

#[test]
fn part_two() -> Result<(), Error> {
    let stones = load_puzzle()?;

    println!("{:?}", stones);

    Ok(())
}