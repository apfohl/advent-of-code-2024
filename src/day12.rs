use crate::common::lines;
use std::io::Error;

fn load_puzzle() -> Result<Vec<Vec<char>>, Error> {
    let puzzle: Vec<Vec<char>> = lines("inputs/day_12_test.txt")?
        .filter_map(|line| match line {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();

    Ok(puzzle)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn dfs(
    crops: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
    crop: char,
) -> (usize, usize) {
    let mut stack = vec![start];

    let mut fields = 0usize;
    let mut edges = 0usize;

    while let Some((x, y)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        fields += 1;

        for (dx, dy) in DIRECTIONS {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if let Some(&c) = crops.get(new_y).and_then(|row| row.get(new_x)) {
                if visited[new_y][new_x] {
                    continue;
                }

                if c == crop {
                    stack.push((new_x, new_y))
                } else {
                    edges += 1
                }
            } else {
                edges += 1
            }
        }

        // DIRECTIONS
        //     .into_iter()
        //     .filter_map(|(dx, dy)| {
        //         let new_x = (x as isize + dx) as usize;
        //         let new_y = (y as isize + dy) as usize;
        //
        //         match crops.get(new_y).and_then(|row| row.get(new_x)) {
        //             Some(&c) if c == crop && !visited[new_y][new_x] => Some((new_x, new_y)),
        //             _ => None,
        //         }
        //     })
        //     .for_each(|point| stack.push(point))
    }

    (fields, edges)
}

fn find_regions(crops: &Vec<Vec<char>>) {
    let mut visited = vec![vec![false; crops[0].len()]; crops.len()];

    for (y, &ref row) in crops.iter().enumerate() {
        for (x, &crop) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }

            let (fields, edges) = dfs(crops, &mut visited, (x, y), crop);
            println!("{crop}: {fields} {edges}")
        }
    }
}

#[test]
fn part_one() -> Result<(), Error> {
    let crops = load_puzzle()?;

    println!("{:?}", crops);

    find_regions(&crops);

    Ok(())
}

#[test]
fn part_two() -> Result<(), Error> {
    Ok(())
}
