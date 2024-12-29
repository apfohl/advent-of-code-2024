use std::collections::HashSet;
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

fn handle_stone(stone: usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        number if number.to_string().len() % 2 == 0 => {
            let string = number.to_string();
            let (left, right) = string.split_at(string.len() / 2);

            vec![
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            ]
        }
        number => vec![number * 2024],
    }
}

#[test]
fn handle_stone_test() {
    vec![
        (0usize, vec![1usize]),
        (23usize, vec![2usize, 3usize]),
        (2usize, vec![4048usize]),
    ]
    .iter()
    .for_each(|(stone, result)| assert_eq!(*result, handle_stone(*stone)))
}

#[test]
fn part_one() -> Result<(), Error> {
    let mut stones = load_puzzle()?;

    println!("{:?}", stones);

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|stone| handle_stone(stone))
            .collect()
    }

    println!("{}", stones.len());

    Ok(())
}

fn dfs(stones: &Vec<usize>) -> usize {
    let mut counter = stones.len();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut stack = stones.clone();

    while let Some(stone) = stack.pop() {
        let new_stones = handle_stone(stone);

        counter += new_stones.len();

        for new_stone in new_stones {
            if visited.contains(&new_stone) {
                continue
            }

            visited.insert(new_stone);
            stack.push(new_stone)
        }
    }

    counter
}

#[test]
fn part_two() -> Result<(), Error> {
    let stones = load_puzzle()?;

    println!("{:?}", stones);

    let count = dfs(&stones);

    println!("{}", count);

    Ok(())
}
