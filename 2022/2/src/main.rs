mod game;
mod utils;

// We can use what the trait provides only if we "use"(import) it
use game::RPSRound;

fn main() {
    let rounds = utils::read_lines("input")
        .expect("failed to open file")
        .as_rps();

    let mut sum: u32 = 0;
    for round in &rounds {
        // round = (HandShape, Outcome)
        sum += round.0 + round.1 // works because we impl'd Add<> trait
    }

    // Debug only for generating the puzzle solution
    for round in rounds {
        println!("{:?}, {:?}", round.0, round.1);
    }
    println!("Total: {}", sum);
}
