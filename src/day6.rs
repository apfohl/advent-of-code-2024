use crate::common::lines;
use std::collections::HashSet;
use std::io::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn step_forward(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn next_location(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (self.x as isize, self.y as isize - 1),
            Direction::Right => (self.x as isize + 1, self.y as isize),
            Direction::Down => (self.x as isize, self.y as isize + 1),
            Direction::Left => (self.x as isize - 1, self.y as isize),
        }
    }
}

fn load_puzzle() -> Result<(usize, usize, Vec<(usize, usize)>, Guard), Error> {
    let mut width = 0;
    let mut height = 0;
    let mut obstructions = Vec::new();
    let mut guard = Guard {
        direction: Direction::Up,
        x: 0,
        y: 0,
    };

    for (y, line) in lines("inputs/day_6.txt")?.enumerate() {
        height = y;

        for (x, character) in line?.chars().enumerate() {
            width = x;

            match character {
                '#' => obstructions.push((x, y)),
                '^' => {
                    guard.x = x;
                    guard.y = y;
                }
                _ => {}
            }
        }
    }

    Ok((width, height, obstructions, guard))
}

fn check_location(
    width: usize,
    height: usize,
    obstructions: &Vec<(usize, usize)>,
    position: (isize, isize),
) -> Option<bool> {
    if position.0 < 0
        || position.0 > width as isize
        || position.1 < 0
        || position.1 > height as isize
    {
        return None;
    }

    if obstructions.iter().any(|&obstruction| {
        obstruction.0 == position.0 as usize && obstruction.1 == position.1 as usize
    }) {
        Some(false)
    } else {
        Some(true)
    }
}

fn calculate_visited() -> Result<HashSet<(usize, usize)>, Error> {
    let (width, height, obstructions, guard) = load_puzzle()?;
    let mut guard = guard;

    let mut running = true;

    let mut visited = HashSet::new();

    while running {
        let _ = visited.insert((guard.x, guard.y));
        let next_location = guard.next_location();

        if let Some(is_free) = check_location(width, height, &obstructions, next_location) {
            if !is_free {
                guard.turn_right();
            }

            guard.step_forward();
        } else {
            running = false;
        }
    }

    Ok(visited)
}

#[test]
fn part_one() -> Result<(), Error> {
    let visited = calculate_visited()?;

    println!("{}", visited.len());

    Ok(())
}

#[test]
fn part_two() -> Result<(), Error> {
    println!("START");
    let (width, height, obstructions, guard) = load_puzzle()?;
    println!("DIMENSIONS: {} {}", width, height);

    let mut obstacles: usize = 0;

    let mut v = calculate_visited()?;
    println!("START REMOVED {}", v.remove(&(guard.x, guard.y)));

    for (i, (x, y)) in v.iter().enumerate() {
        println!("RUN: {i} {x} {y}");
        let mut guard = guard.clone();
        let mut obstructions = obstructions.clone();
        obstructions.push((*x, *y)); // additional obstruction

        let mut visited = HashSet::new();
        let _ = visited.insert(guard.clone());

        loop {
            let next_location = guard.next_location();

            if let Some(is_free) = check_location(width, height, &obstructions, next_location) {
                if !is_free {
                    guard.turn_right();

                    let mut guard_clone = guard.clone();
                    guard_clone.step_forward();
                    if let Some(false) = check_location(
                        width,
                        height,
                        &obstructions,
                        (guard_clone.x as isize, guard_clone.y as isize),
                    ) {
                        guard.turn_right()
                    }
                }

                guard.step_forward();
            } else {
                break;
            }

            if visited.contains(&guard) {
                obstacles += 1;
                break;
            }

            let _ = visited.insert(guard.clone());
        }
    }

    println!("OBSTACLES: {}", obstacles);

    Ok(())
}
