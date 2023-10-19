mod utils;
mod game;

use game::RPSRound;

fn main() {
    let rounds = utils::read_lines("input").expect("failed to open file").as_rps();
    let mut sum: u32 = 0;

    rounds.iter().for_each(|round| sum += round.0 + round.1);
    
    // Debug only for generating the puzzle solution
    for round in rounds {
        println!("{:?}, {:?}", round.0, round.1);
    }
    println!("sum: {}", sum);
}
