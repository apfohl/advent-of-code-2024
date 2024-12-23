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

struct Combinations {
    operators: Vec<char>,
    indices: Vec<usize>,
    done: bool,
}

impl Combinations {
    fn new(operators: Vec<char>, n: usize) -> Self {
        match n {
            0 => Self {
                operators,
                indices: vec![],
                done: true,
            },
            _ => Self {
                operators,
                indices: vec![0; n],
                done: false,
            },
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let combination: Vec<char> = self.indices.iter().map(|&i| self.operators[i]).collect();

        for i in (0..self.indices.len()).rev() {
            if self.indices[i] < self.operators.len() - 1 {
                self.indices[i] += 1;

                for j in i + 1..self.indices.len() {
                    self.indices[j] = 0;
                }

                return Some(combination);
            }
        }

        self.done = true;

        Some(combination)
    }
}

fn check_combination(equation: &Equation, combination: Vec<char>) -> bool {
    equation.result
        == equation
            .operands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, operand)| {
                if i == 0 {
                    return *operand;
                }

                let operator = combination[i - 1];
                match operator {
                    '+' => acc + operand,
                    '*' => acc * operand,
                    '|' => format!("{acc}{operand}").parse::<usize>().unwrap(),
                    _ => panic!("Unknown operator: {operator}"),
                }
            })
}

fn calculate_sum(equations: Vec<Equation>, operators: Vec<char>) -> usize {
    equations
        .iter()
        .filter_map(|equation| {
            match Combinations::new(operators.clone(), equation.operands.len() - 1)
                .into_iter()
                .any(|combination| check_combination(&equation, combination))
            {
                true => Some(equation.result),
                false => None,
            }
        })
        .sum()
}

#[test]
fn part_one() -> Result<(), Error> {
    let equations = load_puzzle()?;
    let operators = vec!['+', '*'];

    let sum: usize = calculate_sum(equations, operators);

    println!("{sum}");

    Ok(())
}

#[test]
fn part_two() -> Result<(), Error> {
    let equations = load_puzzle()?;
    let operators = vec!['+', '*', '|'];

    let sum: usize = calculate_sum(equations, operators);

    println!("{sum}");

    Ok(())
}