mod rucksack;
mod utils;

use rucksack::Rucksack;

fn main() {
    let rucksacks = utils::read_lines("input")
        .expect("failed to open file")
        .as_rucksack();

    // The total score for each rucksack is u8, however the sum can easily
    // overflow it, thus, convert each to u32 for summing
    let sum: u32 = rucksacks.into_iter()
        .map(|rucksack| rucksack.score().1 as u32)
        .sum();
    println!("{}", sum);
}
