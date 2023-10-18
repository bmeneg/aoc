use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Clone, Copy)]
struct Elf {
    id: usize, // we don't really need id, that's just out of curiosity
    calories: i32,
}

fn read_lines(fname: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(fname)?;
    Ok(BufReader::new(file).lines())
}

trait RankedElfs {
    fn ranked_insert(&mut self, value: Elf);
    fn ranked_total(&self) -> i32;
}

impl RankedElfs for Vec<Elf> {
    fn ranked_insert(&mut self, elf: Elf) {
        let v_len = self.len();

        for i in 0..v_len {
            if elf.calories > self[i].calories {
                self.insert(i, elf);
                self.pop();
                return;
            }
        }
    }

    fn ranked_total(&self) -> i32 {
        let mut total: i32 = 0;
        self.iter().for_each(|elf| total += elf.calories);
        return total;
    }
}

fn main() {
    let mut elfs = vec![
        Elf { id: 0, calories: 0 },
        Elf { id: 0, calories: 0 },
        Elf { id: 0, calories: 0 },
    ];
    let mut sum: i32 = 0;
    let lines_iter = read_lines("input").expect("failed to open file");

    for (i, lines) in lines_iter.into_iter().enumerate() {
        if let Ok(line) = lines {
            if line.is_empty() {
                elfs.ranked_insert(Elf {
                    id: i,
                    calories: sum,
                });
                sum = 0;
            } else {
                sum += match line.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => 0,
                };
            }
        }
    }

    // Debug only to generate the answers we need to for AOC
    for elf in &elfs {
        println!("{}: {}", elf.id, elf.calories);
    }
    println!("total: {}", elfs.ranked_total());
}
