use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn read_lines(fname: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(fname)?;
    Ok(BufReader::new(file).lines())
}
