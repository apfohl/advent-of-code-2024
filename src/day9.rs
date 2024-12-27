use crate::common::lines;
use std::io::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Block {
    id: Option<usize>,
}

fn load_puzzle() -> Result<Vec<char>, Error> {
    let puzzle = lines("inputs/day_9.txt")?
        .find_map(|line| match line {
            Ok(line) => Some(line.chars().collect::<Vec<char>>()),
            Err(_) => None,
        })
        .unwrap_or_default();

    Ok(puzzle)
}

fn create_disk(characters: &Vec<char>) -> Vec<Block> {
    let mut disk = vec![];
    let mut file_id_counter = 0usize;

    for (i, &character) in characters.iter().enumerate() {
        if let Some(count) = character.to_digit(10).map(|d| d as usize) {
            if i % 2 == 0 {
                disk.extend(vec![
                    Block {
                        id: Some(file_id_counter),
                    };
                    count
                ]);
                file_id_counter += 1;
            } else {
                disk.extend(vec![Block { id: None }; count])
            }
        }
    }

    disk
}

fn find_free_space(disk: &Vec<Block>) -> Option<usize> {
    disk.iter().position(|&block| block.id == None)
}

fn find_next_block(disk: &Vec<Block>) -> Option<usize> {
    disk.iter().rposition(|&block| block.id != None)
}

fn calculate_checksum(disk: &Vec<Block>) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, &block)| block.id.map(|id| i * id))
        .sum()
}

#[test]
fn part_one() -> Result<(), Error> {
    let characters = load_puzzle()?;
    let mut disk = create_disk(&characters);

    loop {
        match (find_free_space(&disk), find_next_block(&disk)) {
            (Some(free), Some(block)) if free < block => disk.swap(free, block),
            _ => break,
        }
    }

    println!("{}", calculate_checksum(&disk));

    Ok(())
}

fn find_free_space_n(disk: &Vec<Block>, size: usize) -> Option<usize> {
    let mut index = 0usize;
    let mut result_size = 0usize;

    for i in 0..disk.len() {
        if disk[i].id == None {
            result_size += 1;

            if result_size == size {
                break;
            }
        } else {
            index = i + 1;
            result_size = 0;
        }
    }

    if result_size == size {
        Some(index)
    } else {
        None
    }
}

#[test]
fn find_free_space_n_test() {
    let input = vec![
        Block { id: Some(0) },
        Block { id: Some(0) },
        Block { id: None },
        Block { id: Some(1) },
        Block { id: Some(1) },
        Block { id: Some(1) },
        Block { id: None },
        Block { id: None },
        Block { id: Some(2) },
        Block { id: None },
    ];

    assert_eq!(Some(2), find_free_space_n(&input, 1));
    assert_eq!(Some(6), find_free_space_n(&input, 2));
    assert_eq!(None, find_free_space_n(&input, 3));
}

fn find_last_file(disk: &Vec<Block>, last_left: usize) -> (usize, usize, Block) {
    let mut left = last_left;
    let mut right = left + 1;
    let mut current_id = None;

    for i in (0..right).rev() {
        match disk[i].id {
            None => {
                if current_id != None {
                    break;
                }

                right = i + 1;
            }
            Some(id) => match current_id {
                Some(current) => {
                    if current != id {
                        break;
                    }
                }
                None => {
                    current_id = Some(id);

                    right = i + 1;
                }
            },
        }

        left = i;
    }

    (left, right, disk[left])
}

#[test]
fn find_last_file_test() {
    let input = vec![
        Block { id: Some(0) },
        Block { id: Some(0) },
        Block { id: None },
        Block { id: Some(1) },
        Block { id: Some(1) },
        Block { id: Some(1) },
        Block { id: None },
        Block { id: Some(2) },
        Block { id: Some(2) },
        Block { id: None },
    ];

    let (left, right, _) = find_last_file(&input, 5);

    assert_eq!((3, 6), (left, right));
}

fn swap_range(disk: &mut Vec<Block>, range: std::ops::Range<usize>, new_position: usize) {
    if range.start >= disk.len()
        || range.end > disk.len()
        || new_position >= disk.len()
        || new_position > range.start
    {
        return;
    }

    for i in 0..range.len() {
        let temp = disk[new_position + i];
        disk[new_position + i] = disk[range.start + i];
        disk[range.start + i] = temp;
    }
}

#[test]
fn swap_range_test() {
    let mut input = vec![
        Block { id: Some(0) }, // 0
        Block { id: Some(0) }, // 1
        Block { id: None },    // 2
        Block { id: None },    // 3
        Block { id: Some(1) }, // 4
        Block { id: Some(1) }, // 5
        Block { id: None },    // 6
        Block { id: Some(2) }, // 7
        Block { id: Some(2) }, // 8
        Block { id: None },    // 9
    ];

    swap_range(&mut input, 7..9, 2);

    let expected = vec![
        Block { id: Some(0) }, // 0
        Block { id: Some(0) }, // 1
        Block { id: Some(2) }, // 2
        Block { id: Some(2) }, // 3
        Block { id: Some(1) }, // 4
        Block { id: Some(1) }, // 5
        Block { id: None },    // 6
        Block { id: None },    // 7
        Block { id: None },    // 8
        Block { id: None },    // 9
    ];

    assert_eq!(input, expected);
}

#[test]
fn part_two() -> Result<(), Error> {
    let characters = load_puzzle()?;
    let mut disk = create_disk(&characters);

    let mut last_left = disk.len() - 1;

    loop {
        let (left, right, _) = find_last_file(&disk, last_left);

        if let Some(free) = find_free_space_n(&disk, right - left) {
            swap_range(&mut disk, left..right, free);
        }

        if left == 0 {
            break;
        }

        last_left = left - 1;
    }

    println!("{}", calculate_checksum(&disk));

    Ok(())
}
