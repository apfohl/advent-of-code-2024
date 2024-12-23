use crate::common::lines;
use std::collections::HashMap;
use std::io::Error;

fn load_puzzle() -> Result<Vec<Vec<char>>, Error> {
    let puzzle: Vec<Vec<char>> = lines("inputs/day_8.txt")?
        .filter_map(|line| match line {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();

    Ok(puzzle)
}

fn add_or_insert(
    antennas: &mut HashMap<char, Vec<(usize, usize)>>,
    character: char,
    x: usize,
    y: usize,
) {
    antennas
        .entry(character)
        .or_insert_with(Vec::new)
        .push((x, y))
}

fn get_antennas(puzzle: &mut Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            match puzzle[y][x] {
                '.' => continue,
                character => add_or_insert(&mut antennas, character, x, y),
            }
        }
    }

    antennas
}

fn load_antennas() -> Result<
    (
        Vec<Vec<char>>,
        (usize, usize),
        HashMap<char, Vec<(usize, usize)>>,
    ),
    Error,
> {
    let mut puzzle = load_puzzle()?;
    let dimensions = (puzzle[0].len(), puzzle.len());

    let antennas = get_antennas(&mut puzzle);

    Ok((puzzle, dimensions, antennas))
}

fn calculate_antinodes_one(
    lhs: (usize, usize),
    rhs: (usize, usize),
    dimensions: (usize, usize),
) -> Vec<(usize, usize)> {
    let (x1, y1) = lhs;
    let (x2, y2) = rhs;

    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;

    vec![
        ((x1 as isize - dx) as usize, (y1 as isize - dy) as usize),
        ((x2 as isize + dx) as usize, (y2 as isize + dy) as usize),
    ]
    .into_iter()
    .filter(|&(x, y)| x < dimensions.0 && y < dimensions.1)
    .collect()
}

#[test]
fn test_calculate_antinodes() {
    let cases = vec![
        (
            (5usize, 2usize),
            (4usize, 4usize),
            vec![(6usize, 0usize), (3usize, 6usize)],
        ),
        ((8usize, 8usize), (11usize, 11usize), vec![(5usize, 5usize)]),
    ];

    cases.iter().for_each(|(lhs, rhs, expected)| {
        let result = calculate_antinodes_one(*lhs, *rhs, (12, 12));
        assert_eq!(result, *expected);
    })
}

fn calculate_antinodes_for_antennas<F>(
    points: &Vec<(usize, usize)>,
    dimensions: (usize, usize),
    func: &F,
) -> Vec<(usize, usize)>
where
    F: Fn((usize, usize), (usize, usize), (usize, usize)) -> Vec<(usize, usize)>,
{
    if points.is_empty() {
        return vec![];
    }

    let lhs = points[0];
    let tail = &points[1..].to_vec();

    tail.into_iter()
        .flat_map(|rhs| func(lhs, *rhs, dimensions))
        .chain(calculate_antinodes_for_antennas(tail, dimensions, func).into_iter())
        .collect()
}

#[test]
fn test_calculate_antinodes_for_antennas() {
    let antennas = vec![(8, 1), (5, 2), (7, 3), (4, 4)];

    let antinodes = calculate_antinodes_for_antennas(&antennas, (12, 12), &calculate_antinodes_one);
    let expected = vec![
        (11, 0),
        (2, 3),
        (6, 5),
        (0, 7),
        (3, 1),
        (9, 4),
        (6, 0),
        (3, 6),
        (10, 2),
        (1, 5),
    ];

    assert_eq!(antinodes, expected);
}

#[test]
fn part_one() -> Result<(), Error> {
    let (mut puzzle, dimensions, antennas) = load_antennas()?;

    for (_, points) in antennas {
        calculate_antinodes_for_antennas(&points, dimensions, &calculate_antinodes_one)
            .into_iter()
            .for_each(|(x, y)| puzzle[y][x] = '#')
    }

    let count = puzzle.iter().flatten().filter(|&&c| c == '#').count();
    println!("{count}");

    Ok(())
}

fn calculate_antinodes_two(
    lhs: (usize, usize),
    rhs: (usize, usize),
    dimensions: (usize, usize),
) -> Vec<(usize, usize)> {
    let (x1, y1) = lhs;
    let (x2, y2) = rhs;

    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;

    let mut points = Vec::new();

    let mut x = x1 as isize;
    let mut y = y1 as isize;

    while x >= 0 && x < dimensions.0 as isize && y >= 0 && y < dimensions.1 as isize {
        points.push((x as usize, y as usize));
        x -= dx;
        y -= dy;
    }

    x = x2 as isize;
    y = y2 as isize;

    while x >= 0 && x < dimensions.0 as isize && y >= 0 && y < dimensions.1 as isize {
        points.push((x as usize, y as usize));
        x += dx;
        y += dy;
    }

    points
}

#[test]
fn part_two() -> Result<(), Error> {
    let (mut puzzle, dimensions, antennas) = load_antennas()?;

    for (_, points) in antennas {
        calculate_antinodes_for_antennas(&points, dimensions, &calculate_antinodes_two)
            .into_iter()
            .for_each(|(x, y)| puzzle[y][x] = '#')
    }

    let count = puzzle.iter().flatten().filter(|&&c| c == '#').count();
    println!("{count}");

    Ok(())
}
