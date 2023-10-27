use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Buffered read in a specific file to decrease the total number to read()
// syscall. Return a line iterator as Lines<> -> Result<String, Error>.
pub fn read_lines(fname: &str) -> io::Lines<BufReader<File>> {
    let file = match File::open(fname) {
        Ok(f) => f,
        Err(e) => panic!("error opening file: {}", e),
    };
    BufReader::new(file).lines()
}
