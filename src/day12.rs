use crate::common::lines;
use std::io::Error;

fn load_puzzle() -> Result<Vec<Vec<char>>, Error> {
    let puzzle: Vec<Vec<char>> = lines("inputs/day_12.txt")?
        .filter_map(|line| match line {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();

    Ok(puzzle)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const CORNERS: [[(isize, isize); 3]; 4] = [
    [(0, -1), (1, -1), (1, 0)],
    [(1, 0), (1, 1), (0, 1)],
    [(0, 1), (-1, 1), (-1, 0)],
    [(-1, 0), (-1, -1), (0, -1)],
];

fn count_corners(crops: &Vec<Vec<char>>, position: (usize, usize), plant: char) -> usize {
    let mut corners = vec![];

    for corner in CORNERS {
        let mut new_corner = vec![];
        for (x, y) in corner {
            new_corner.push((
                (position.0 as isize + x) as usize,
                (position.1 as isize + y) as usize,
            ))
        }

        corners.push(new_corner);
    }

    let mut result = 0usize;

    corners.iter().for_each(|corner| {
        let points = corner
            .into_iter()
            .map(
                |&(x, y)| match crops.get(y).and_then(|row| row.get(x)) {
                    Some(&p) if p == plant => true,
                    _ => false,
                },
            )
            .collect::<Vec<bool>>();

        match points[..] {
            [false, _, false] => result += 1,
            [true, false, true] => result += 1,
            _ => {}
        }
    });

    result
}

fn dfs(
    crops: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
    crop: char,
) -> (usize, usize, usize) {
    let mut stack = vec![start];

    let mut fields = 0usize;
    let mut edges = 0usize;
    let mut corners = 0usize;

    while let Some((x, y)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        fields += 1;
        corners += count_corners(crops, (x, y), crops[y][x]);

        for (dx, dy) in DIRECTIONS {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if let Some(&col) = crops.get(new_y).and_then(|row| row.get(new_x)) {
                match col {
                    c if c != crop => edges += 1,
                    c if c == crop && !visited[new_y][new_x] => stack.push((new_x, new_y)),
                    _ => continue,
                }
            } else {
                // out of bounds
                edges += 1
            }
        }
    }

    (fields, edges, corners)
}

fn find_regions(crops: &Vec<Vec<char>>) {
    let mut visited = vec![vec![false; crops[0].len()]; crops.len()];

    let mut price_one = 0usize;
    let mut price_two = 0usize;

    for (y, &ref row) in crops.iter().enumerate() {
        for (x, &crop) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }

            let (fields, edges, corners) = dfs(crops, &mut visited, (x, y), crop);
            price_one += fields * edges;
            price_two += fields * corners;

            // println!("{crop}: {fields} {edges} {corners}")
        }
    }

    println!("Part one: {price_one}");
    println!("Part two: {price_two}")
}

#[test]
fn solve() -> Result<(), Error> {
    let crops = load_puzzle()?;

    find_regions(&crops);

    Ok(())
}
