#![allow(dead_code)]

mod elves;
mod utils;

use elves::{AsRucksack, Elf, Group};

fn compute_score(vec: &Vec<u8>) -> u32 {
    vec.iter()
        .map(|v| {
            if v.is_ascii_lowercase() {
                // ASCII lowercase starts at 97, but scores are counted
                // from 1 to 26
                (v - 97 + 1) as u32
            } else {
                // ASCII uppercase starts at 65, but scores are counted
                // from 27 to 52
                (v - 65 + 27) as u32
            }
        })
        .sum::<u32>()
}

fn main() {
    let rucksacks = utils::read_lines("input").as_rucksack();
    let elves = Elf::from_rucksacks(rucksacks);
    let groups = Group::from_elves(elves);
    let badges: Vec<u8> = groups.iter()
        .map(|g| g.borrow().badge.unwrap())
        .collect();

    println!("badges total: {}", compute_score(&badges));
}
