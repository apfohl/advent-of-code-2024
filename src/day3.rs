use crate::common;
use regex::Regex;
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug)]
enum AocError {
    Error(Error),
    Regex(regex::Error),
    ParseInt(ParseIntError),
}

impl From<Error> for AocError {
    fn from(err: Error) -> AocError {
        AocError::Error(err)
    }
}

impl From<regex::Error> for AocError {
    fn from(err: regex::Error) -> AocError {
        AocError::Regex(err)
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> AocError {
        AocError::ParseInt(err)
    }
}

fn convert(input: &str) -> Result<u32, AocError> {
    input.parse::<u32>().map_err(AocError::ParseInt)
}

#[test]
fn part_one() -> Result<(), AocError> {
    let lines = common::lines("inputs/day_3.txt").map_err(AocError::Error)?;
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").map_err(AocError::Regex)?;

    let sum = lines
        .map(|line| match line {
            Ok(line) => regex
                .captures_iter(line.as_ref())
                .filter_map(|capture| {
                    let (_, [x, y]) = capture.extract();

                    let x = convert(x);
                    let y = convert(y);

                    match (x, y) {
                        (Ok(x), Ok(y)) => Some(x * y),
                        _ => None,
                    }
                })
                .sum::<u32>(),
            Err(_) => 0,
        })
        .sum::<u32>();

    println!("{}", sum);

    Ok(())
}

#[test]
fn part_two() -> Result<(), AocError> {
    let mut lines = common::lines("inputs/day_3.txt").map_err(AocError::Error)?;
    let regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").map_err(AocError::Regex)?;

    let mut skip = false;

    let sum = lines
        .map(|line| match line {
            Ok(line) => regex
                .captures_iter(line.as_str())
                .filter_map(|capture| {
                    if capture[0].eq("do()") {
                        skip = false
                    } else if capture[0].eq("don't()")  {
                        skip = true
                    } else {
                        return if skip {
                            None
                        } else {
                            let x = convert(&capture[1]);
                            let y = convert(&capture[2]);

                            match (x, y) {
                                (Ok(x), Ok(y)) => Some(x * y),
                                _ => None,
                            }
                        }
                    }

                    None
                })
                .sum::<u32>(),
            Err(_) => 0,
        })
        .sum::<u32>();

    println!("{}", sum);

    Ok(())
}
