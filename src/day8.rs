use std::io::Error;
use crate::common::lines;

fn load_puzzle() -> Result<Vec<Vec<char>>, Error> {
    let puzzle: Vec<Vec<char>> = lines("inputs/day_8_test.txt")?
        .filter_map(|line| match line {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();

    Ok(puzzle)
}

#[test]
fn part_one() -> Result<(), Error> {
    let puzzle = load_puzzle()?;

    println!("{:?}", puzzle);

    Ok(())
}