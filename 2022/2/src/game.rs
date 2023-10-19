// The game is a simple Rock, Paper, Scissors (RPS).
// And for that we have a struct to define the Game with its respective
// rules, which is instantiated with the ::new() function and also a
// ::lookup() function for retrieving what's the correct hand shape for the
// desired outcome.

use std::fs::File;
use std::io::{self, BufReader};
use std::ops::Add;

// What hand shapes are possible
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Unknown = 0,
}

// The puzzle answer is the amount of points we score at the end of all
// rounds, thus implementing the Add trait for the enum can be quite handy
// later on
impl Add<Outcome> for HandShape {
    // We're using u32 representation for enums instead of the default isize
    type Output = u32;

    fn add(self, other: Outcome) -> Self::Output {
        // Numeric casting an enum gets its discriminant value
        // (EnumItem = <discriminant>)
        (self as Self::Output) + (other as Self::Output)
    }
}

// What outcomes are possible for the game. We use it for lookup the best
// hand for a specific outcome
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

// The exact same goal as the Add trait implementation done before, but here
// we do the other way around, so we can safely change the addition order
impl Add<HandShape> for Outcome {
    type Output = u32;

    fn add(self, other: HandShape) -> Self::Output {
        (self as Self::Output) + (other as Self::Output)
    }
}

// A simple struct to define the rules for RPS and also for using later for
// looking up the correct hand for a specific outcome
struct HandRule {
    hand: HandShape,
    loses_to: HandShape,
}

// Well, our game doesn't have much other than a set of rules, a constructor
// and the lookup function
struct RPSGame {
    rules: [HandRule; 3], // RPS rules are immutable
}

impl RPSGame {
    fn new() -> Self {
        Self {
            rules: [
                HandRule {
                    hand: HandShape::Rock,
                    loses_to: HandShape::Paper,
                },
                HandRule {
                    hand: HandShape::Paper,
                    loses_to: HandShape::Scissors,
                },
                HandRule {
                    hand: HandShape::Scissors,
                    loses_to: HandShape::Rock,
                },
            ],
        }
    }

    // The part 2 of AoC puzzle asks for the best hand shape for a specific
    // outcome based on the other player's (lhs: left-hand-side) hand shape
    fn lookup(&self, lhs: HandShape, outcome: Outcome) -> HandShape {
        let mut hand = HandShape::Unknown;

        for rule in &self.rules {
            if outcome == Outcome::Win {
                if lhs == rule.hand {
                    hand = rule.loses_to;
                }
            } else if outcome == Outcome::Draw {
                hand = lhs;
            } else {
                if lhs == rule.loses_to {
                    hand = rule.hand;
                }
            }
        }

        return hand;
    }
}

// We're totally over engineering things here: we don't need to implement a
// specific trait for the io::Lines<> interator, we could move all this
// logic to the main function or, at least, a couple different functions not
// related to the interator at all. However, in order to better understand
// traits, I've decided to get the io::Lines<> iterator returned by
// utils::read_lines() (called at the main function) and convert all the
// input data into game's specific/useful data.
//
// With that, in the main function we can basically call
// utils::read_lines().as_rps() and calculate the final score over the final
// (YOUR_HandShape, DEFINED_Outcome) tuple directly.
//
// Yes, that's somewhat overkill, but at the same time seems a nice
// abstraction IHMO ;)
pub trait RPSRound {
    fn as_rps(&mut self) -> Vec<(HandShape, Outcome)>;
}

// Implement the trait specifically for the utils::read_lines() return
// iterator, avoiding any unnecessary generic code.
impl RPSRound for io::Lines<BufReader<File>> {
    fn as_rps(&mut self) -> Vec<(HandShape, Outcome)> {
        let game = RPSGame::new();
        let mut rounds: Vec<(HandShape, Outcome)> = vec![];

        for line_res in self.into_iter() {
            // input file format: "<other's hand> <predefined outcome>"
            if let Ok(line) = line_res {
                let outcome: Outcome;
                let mut line_parts = line.split_whitespace();

                let other_hand = match line_parts.next().unwrap() {
                    "A" => HandShape::Rock,
                    "B" => HandShape::Paper,
                    "C" => HandShape::Scissors,
                    _ => continue,
                };

                // choose our hand based on other's hand and the predefined
                // round outcome (should we win, draw or lose this round?)
                let your_hand = match line_parts.next().unwrap() {
                    "X" => {
                        outcome = Outcome::Loss;
                        game.lookup(other_hand, outcome)
                    }
                    "Y" => {
                        outcome = Outcome::Draw;
                        game.lookup(other_hand, outcome)
                    }
                    "Z" => {
                        outcome = Outcome::Win;
                        game.lookup(other_hand, outcome)
                    }
                    _ => continue,
                };

                // we return this touple so we can calculate the final
                // score, which depends on our hand and the round outcome
                rounds.push((your_hand, outcome));
            }
        }

        return rounds;
    }
}
