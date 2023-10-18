mod utils;
mod elves;

use elves::{
    Elf,
    RankedElves,
};

fn main() {
    let mut elves = vec![
        Elf { id: 0, calories: 0 },
        Elf { id: 0, calories: 0 },
        Elf { id: 0, calories: 0 },
    ];
    let mut sum: i32 = 0;
    let lines_iter = utils::read_lines("input").expect("failed to open file");

    for (i, lines) in lines_iter.into_iter().enumerate() {
        if let Ok(line) = lines {
            if line.is_empty() {
                elves.elves_rank_insert(Elf {
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
    for elf in &elves {
        println!("{}: {}", elf.id, elf.calories);
    }
    println!("total: {}", elves.elves_rank_total());
}
