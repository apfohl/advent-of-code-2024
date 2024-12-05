use crate::common::lines;
use std::io::Error;

trait VecVecCharExt {
    fn get_safe(&self, x: i32, y: i32) -> Option<char>;
}

impl VecVecCharExt for Vec<Vec<char>> {
    fn get_safe(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get(y as usize)
                .and_then(|row| row.get(x as usize).copied())
        }
    }
}

fn load_puzzle() -> Result<Vec<Vec<char>>, Error> {
    let puzzle: Vec<Vec<char>> = lines("inputs/day_4.txt")?
        .filter_map(|line| match line {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();

    Ok(puzzle)
}

fn get_words(puzzle: &Vec<Vec<char>>, x: i32, y: i32) -> Vec<String> {
    let positions = [
        [(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
        [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
        [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
        [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
        [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
        [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
        [(x, y), (x - 1, y), (x - 2, y), (x - 3, y)],
        [(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
    ];

    let mut results = Vec::new();

    for j in 0..positions.len() {
        results.push(
            positions[j]
                .iter()
                .filter_map(|(m, n)| puzzle.get_safe(*m, *n))
                .collect::<Vec<char>>()
                .iter()
                .collect::<String>(),
        )
    }

    results
}

#[test]
fn part_one() -> Result<(), Error> {
    let puzzle = load_puzzle()?;

    println!("{:?}", puzzle);

    let mut sum: usize = 0;

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            if puzzle.get_safe(x as i32, y as i32) == Some('X') {
                let words = get_words(&puzzle, x as i32, y as i32);

                let i = words
                    .iter()
                    .filter_map(|word| if word == "XMAS" { Some(word) } else { None })
                    .count();

                println!("WORDS: {:?} - COUNT: {}", words, i);

                sum += i;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}

fn get_mas(puzzle: &Vec<Vec<char>>, x: i32, y: i32) -> Vec<String> {
    let positions = [
        [
            (x, y, 'A'),
            (x + 1, y + 1, 'M'),
            (x + 1, y - 1, 'M'),
            (x - 1, y - 1, 'S'),
            (x - 1, y + 1, 'S'),
        ],
        [
            (x, y, 'A'),
            (x + 1, y + 1, 'S'),
            (x + 1, y - 1, 'M'),
            (x - 1, y - 1, 'M'),
            (x - 1, y + 1, 'S'),
        ],
        [
            (x, y, 'A'),
            (x + 1, y + 1, 'S'),
            (x + 1, y - 1, 'S'),
            (x - 1, y - 1, 'M'),
            (x - 1, y + 1, 'M'),
        ],
        [
            (x, y, 'A'),
            (x + 1, y + 1, 'M'),
            (x + 1, y - 1, 'S'),
            (x - 1, y - 1, 'S'),
            (x - 1, y + 1, 'M'),
        ],
    ];

    let mut results = Vec::new();

    for j in 0..positions.len() {
        results.push(
            positions[j]
                .iter()
                .filter_map(|(m, n, c)| {
                    //println!("FILTER: {m}|{n}|{c}");
                    puzzle
                        .get_safe(*m, *n)
                        .and_then(|character| if character.eq(c) { Some(*c) } else { None })
                })
                .collect::<Vec<char>>()
                .iter()
                .collect::<String>()
        )
    }

    results
}

#[test]
fn part_two() -> Result<(), Error> {
    let puzzle = load_puzzle()?;

    let mut sum: usize = 0;

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            if puzzle.get_safe(x as i32, y as i32) == Some('A') {
                let words = get_mas(&puzzle, x as i32, y as i32);

                let i = words
                    .iter()
                    .filter_map(|word| if word.chars().count() == 5 { Some(word) } else { None })
                    .count();

                sum += i;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
