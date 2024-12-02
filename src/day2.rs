use crate::common;
use std::io::Error;

enum Direction {
    Start,
    Increasing,
    Decreasing,
}

#[test]
fn part_one() -> Result<(), Error> {
    let mut count: u32 = 0;

    let lines = common::lines("inputs/day_2.txt")?;
    for line in lines {
        let numbers: Vec<u32> = line?
            .split_whitespace()
            .map(|part| part.parse().expect("Not a valid number"))
            .collect();

        let result = numbers
            .iter()
            .fold((0, Direction::Start, true), |last, &cur| {
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
            });

        if result.2 == true {
            count += 1
        }
    }

    println!("{}", count);

    Ok(())
}
