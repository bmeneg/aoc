#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufReader};
use std::ops::Add;

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Unknown = 0,
}

impl Add<Outcome> for HandShape {
    type Output = u32;
    fn add(self, other: Outcome) -> Self::Output {
        (self as Self::Output) + (other as Self::Output)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Add<HandShape> for Outcome {
    type Output = u32;
    fn add(self, other: HandShape) -> Self::Output {
        (self as Self::Output) + (other as Self::Output)
    }
}

// Read input file as actual Rock, Paper, Scissors game
pub trait RPSRound {
    fn as_rps(&mut self) -> Vec<(HandShape, Outcome)>;
}

impl RPSRound for io::Lines<BufReader<File>> {
    fn as_rps(&mut self) -> Vec<(HandShape, Outcome)> {
        let mut rounds: Vec<(HandShape, Outcome)> = vec![];
        let mut outcome: Outcome;

        for line_res in self.into_iter() {
            if let Ok(line) = line_res {
                let mut line_parts = line.split_whitespace();
                let other_hand = match line_parts.next().unwrap() {
                    "A" => HandShape::Rock,
                    "B" => HandShape::Paper,
                    "C" => HandShape::Scissors,
                    _ => HandShape::Unknown,
                };
                let your_hand = match line_parts.next().unwrap() {
                    "X" => HandShape::Rock,
                    "Y" => HandShape::Paper,
                    "Z" => HandShape::Scissors,
                    _ => HandShape::Unknown,
                };
                
                if (your_hand == HandShape::Rock && other_hand == HandShape::Scissors) ||
                    (your_hand == HandShape::Paper && other_hand == HandShape::Rock) ||
                    (your_hand == HandShape::Scissors && other_hand == HandShape::Paper)                    
                {
                    outcome = Outcome::Win;
                } else if your_hand == other_hand {
                    outcome = Outcome::Draw;
                } else {
                    outcome = Outcome::Loss;
                }

                rounds.push((your_hand, outcome));
            }
        }

        return rounds;
    }
}
