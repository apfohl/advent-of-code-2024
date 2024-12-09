use crate::common::lines;
use std::io::Error;

#[derive(Debug, Clone)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
}

fn load_puzzle() -> Result<Vec<Equation>, Error> {
    let mut equations: Vec<Equation> = Vec::new();

    for line in lines("inputs/day_7.txt")? {
        let line = line?;

        let mut colon_split = line.split(':');

        let result = colon_split
            .next()
            .and_then(|string| string.parse::<usize>().ok());

        let operands = colon_split.next().map(|string| {
            string
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        });

        result
            .zip(operands)
            .map(|(result, operands)| Equation { result, operands })
            .iter()
            .for_each(|equation| equations.push(equation.clone()));
    }

    Ok(equations)
}

#[test]
fn part_one() -> Result<(), Error> {
    let equations = load_puzzle()?;
    println!("{:?}", equations);

    Ok(())
}
