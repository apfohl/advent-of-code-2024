use crate::common::lines;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

fn as_u32(input: &str) -> Result<usize, Error> {
    input
        .parse::<usize>()
        .map_err(|e| Error::new(InvalidData, e))
}

fn load_puzzle() -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>), Error> {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut parsing_rules = true;

    for line in lines("inputs/day_5.txt")? {
        let line = line?;

        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let r = line
                .split('|')
                .into_iter()
                .filter_map(|value| as_u32(value).ok())
                .collect::<Vec<usize>>();

            rules.push((r[0], r[1]));
        } else {
            updates.push(
                line.split(',')
                    .into_iter()
                    .filter_map(|value| as_u32(value).ok())
                    .collect::<Vec<usize>>(),
            );
        }
    }

    Ok((rules, updates))
}

#[test]
fn part_one() -> Result<(), Error> {
    let (rules, updates) = load_puzzle()?;

    let mut sum: usize = 0;

    for update in updates {
        let mut matches = 0;

        for i in 0..update.len() - 1 {
            let values = rules
                .clone()
                .into_iter()
                .filter_map(|rule| {
                    if rule.0.eq(&update[i]) {
                        Some(rule)
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>();

            if values.into_iter().any(|value| value.1 == update[i + 1]) {
                matches += 1;
            } else {
                break;
            }
        }

        if matches == update.len() - 1 {
            sum += update[update.len() / 2]
        }
    }

    println!("{sum}");

    Ok(())
}

fn swap(vector: Vec<usize>, x: usize, y: usize) -> Vec<usize> {
    let mut vec = vector.clone();
    let value_y = vec[y];
    vec[y] = vec[x];
    vec[x] = value_y;
    vec
}

fn filter_rules(rules: &Vec<(usize, usize)>, second: usize) -> Vec<(usize, usize)> {
    rules
        .clone()
        .into_iter()
        .filter_map(|rule| if rule.0.eq(&second) { Some(rule) } else { None })
        .collect::<Vec<(usize, usize)>>()
}

#[test]
fn part_two() -> Result<(), Error> {
    let (rules, updates) = load_puzzle()?;

    let mut fixed = Vec::new();

    for update in updates {
        let mut update = update;
        let mut broken = false;

        let mut i = 0;

        while i < update.len() - 1 {
            let rules = filter_rules(&rules, update[i]);

            if !rules.into_iter().any(|value| value.1 == update[i + 1]) {
                broken = true;
                update = swap(update, i, i + 1);

                i = 0;
            } else {
                i += 1;
            }
        }

        if broken {
            fixed.push(update.clone());
        }
    }

    println!(
        "{}",
        fixed
            .into_iter()
            .map(|update| update[update.len() / 2])
            .sum::<usize>()
    );

    Ok(())
}
