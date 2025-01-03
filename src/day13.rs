use crate::common::lines;
use itertools::Itertools;
use regex::Regex;
use std::io::Error;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }

    fn parse_button(input: String) -> Option<Self> {
        let regex = Regex::new(r"Button \w: X\+(\d+), Y\+(\d+)").ok()?;
        let captures = regex.captures(&input)?;
        Some(Point::new(
            captures[1].parse().ok()?,
            captures[2].parse().ok()?,
        ))
    }

    fn parse_prize(input: String) -> Option<Self> {
        let regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").ok()?;
        let captures = regex.captures(&input)?;
        Some(Point::new(
            captures[1].parse().ok()?,
            captures[2].parse().ok()?,
        ))
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Game {
    a: Point,
    b: Point,
    prize: Point,
}

impl Game {
    fn new(input: (String, String, String)) -> Option<Self> {
        Point::parse_button(input.0)
            .zip(Point::parse_button(input.1))
            .zip(Point::parse_prize(input.2))
            .map(|((a, b), prize)| Game { a, b, prize })
    }
}

fn load_puzzle() -> Result<Vec<Game>, Error> {
    Ok(lines("inputs/day_13_test.txt")?
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .tuples()
        .filter_map(|(a, b, prize)| Game::new((a, b, prize)))
        .collect())
}

#[test]
fn part_one() -> Result<(), Error> {
    let games = load_puzzle()?;

    println!("{:?}", games);

    Ok(())
}
