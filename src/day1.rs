use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[test]
fn part_1() -> io::Result<()> {
    let file = File::open("inputs/day_1.txt")?;
    let reader = BufReader::new(file);

    let mut col_one = Vec::new();
    let mut col_two = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("   ").collect();

        let numbers: Vec<u32> = parts
            .iter()
            .map(|part| part.parse().expect("Not a valid number"))
            .collect();

        col_one.push(numbers[0]);
        col_two.push(numbers[1]);
    }

    col_one.sort();
    col_two.sort();

    let mut sum: u32 = 0;

    for i in 0..col_one.len() {
        sum += col_one[i].abs_diff(col_two[i])
    }

    println!("{}", sum);

    let mut sum: u32 = 0;

    for value in col_one {
        let count = col_two.iter().filter(|&&x| x == value).count();
        sum += value * count as u32;
    }

    println!("{}", sum);

    Ok(())
}
