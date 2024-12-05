use crate::common;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

enum Direction {
    Start,
    Increasing,
    Decreasing,
}

fn check(numbers: Vec<u32>) -> (u32, Direction, bool) {
    numbers
        .into_iter()
        .fold((0, Direction::Start, true), |last, cur| {
            if last.2 == false {
                return last;
            }

            if last.0 == 0 {
                return (cur, Direction::Start, true);
            }

            let diff = cur.abs_diff(last.0);

            if diff < 1 || diff > 3 {
                return (cur, last.1, false);
            }

            match last.1 {
                Direction::Start => {
                    if cur < last.0 {
                        (cur, Direction::Decreasing, last.2)
                    } else {
                        (cur, Direction::Increasing, last.2)
                    }
                }
                Direction::Increasing => {
                    if cur < last.0 {
                        (cur, Direction::Decreasing, false)
                    } else {
                        (cur, last.1, last.2)
                    }
                }
                Direction::Decreasing => {
                    if cur < last.0 {
                        (cur, last.1, last.2)
                    } else {
                        (cur, Direction::Increasing, false)
                    }
                }
            }
        })
}

#[test]
fn part_one() -> Result<(), Error> {
    let count = common::lines("inputs/day_2.txt")?
        .map(|line| {
            line.and_then(|line| {
                line.split_whitespace()
                    .map(|part| part.parse::<u32>().map_err(|e| Error::new(InvalidData, e)))
                    .collect()
            })
        })
        .map(|vector| vector.map(check).map(|tuple| tuple.2))
        .filter(|valid| match valid {
            Ok(v) => *v,
            Err(_) => false,
        })
        .count();

    println!("{}", count);

    Ok(())
}

fn remove_nth(vec: &Vec<u32>, n: usize) -> Vec<u32> {
    vec.iter()
        .enumerate()
        .filter_map(|(i, x)| if i == n { None } else { Some(x.clone()) })
        .collect()
}

fn create_set(vector: Vec<u32>) -> Vec<Vec<u32>> {
    (0..vector.len()).fold(Vec::new(), |mut set, i| {
        set.push(remove_nth(&vector, i));
        set
    })
}

fn check2(sets: Vec<Vec<u32>>) -> bool {
    sets.iter().any(|set| check(set.to_vec()).2)
}

#[test]
fn part_two() -> Result<(), Error> {
    let count = common::lines("inputs/day_2.txt")?
        .map(|line| {
            line.and_then(|line| {
                line.split_whitespace()
                    .map(|part| part.parse::<u32>().map_err(|e| Error::new(InvalidData, e)))
                    .collect()
            })
        })
        .map(|vector| vector.map(create_set).map(check2))
        .filter(|valid| match valid {
            Ok(v) => *v,
            Err(_) => false,
        })
        .count();

    println!("{}", count);

    Ok(())
}
