use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

pub fn lines(path: &str) -> Result<Lines<BufReader<File>>, Error> {
    File::open(path)
        .map(BufReader::new)
        .map(BufRead::lines)
}
