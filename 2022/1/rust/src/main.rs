use std::io::{
    prelude::*,
    self,
    BufReader
};
use std::fs::File;

#[derive(Debug)]
struct Elf {
    id: usize,
    calories: i32,
}

fn read_lines(fname: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(fname)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut elf_greater = Elf {id: 0, calories: 0};
    let mut sum: i32 = 0;
    let lines_iter = read_lines("input").expect("failed to open file");

    for (i, lines) in lines_iter.into_iter().enumerate() {
        if let Ok(line) = lines {
            if line.is_empty() {
                if sum > elf_greater.calories {
                    elf_greater = Elf{
                        id: i,
                        calories: sum
                    }
                }
                sum = 0;
            } else {
                sum += match line.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => 0,
                };
            }
        }
    }

    println!("{}: {}", elf_greater.id+1, elf_greater.calories);
}
