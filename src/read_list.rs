use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_targets_from_file<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();
    lines
}
