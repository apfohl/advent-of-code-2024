use crate::common::lines;
use std::collections::HashSet;
use std::io::Error;

fn load_puzzle() -> Result<Vec<Vec<i8>>, Error> {
    Ok(lines("inputs/day_10.txt")?
        .filter_map(|line| match line {
            Ok(line) => Some(
                line.chars()
                    .filter_map(|char| char.to_digit(10).map(|digit| digit as i8))
                    .collect(),
            ),
            Err(_) => None,
        })
        .collect())
}

fn directions(map: &Vec<Vec<i8>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let current_height = map[y][x];

    let mut directions = vec![];

    if y as isize - 1 >= 0 && map[y - 1][x] - current_height == 1 {
        directions.push((x, y - 1))
    }

    if y + 1 < map.len() && map[y + 1][x] - current_height == 1 {
        directions.push((x, y + 1))
    }

    if x as isize - 1 >= 0 && map[y][x - 1] - current_height == 1 {
        directions.push((x - 1, y))
    }

    if x + 1 < map[0].len() && map[y][x + 1] - current_height == 1 {
        directions.push((x + 1, y))
    }

    directions
}

fn walk_paths(map: &Vec<Vec<i8>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = start;

    if map[y][x] == 9 {
        return vec![start];
    }

    directions(map, start)
        .iter()
        .map(|&point| walk_paths(&map, point))
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<(usize, usize)>>()
}

#[test]
fn part_one() -> Result<(), Error> {
    let map = load_puzzle()?;

    let mut trailheads = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                trailheads.push(walk_paths(&map, (x, y)).len())
            }
        }
    }

    println!("{}", trailheads.iter().sum::<usize>());

    Ok(())
}

fn walk_paths_two(map: &Vec<Vec<i8>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = start;

    if map[y][x] == 9 {
        return vec![start];
    }

    directions(map, start)
        .iter()
        .map(|&point| walk_paths_two(&map, point))
        .flatten()
        .collect::<Vec<(usize, usize)>>()
}

#[test]
fn part_two() -> Result<(), Error> {
    let map = load_puzzle()?;

    let mut scores = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                scores.extend(walk_paths_two(&map, (x, y)))
            }
        }
    }

    println!("{}", scores.len());

    Ok(())
}
