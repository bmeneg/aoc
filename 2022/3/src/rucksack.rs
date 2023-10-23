use std::fs::File;
use std::io::{self, BufReader};

// Instead of Vec<> we're using Box<> just to test it out.
// Internally Vec<>'s are created in the heap and are also Boxes, but in
// order to get a better understanding of how Rust uses the heap, I created
// a raw Box.
// NOTE: it's too abstract already, didn't feel any difference from Vec<>.
//
// Each rucksack has two different compartiments, where items are placed
// and which has a priority assigned to each (defined by the letter).
#[derive(Debug)]
pub struct Compartments {
    one: Box<[u8]>,
    two: Box<[u8]>,
}

impl Compartments {
    // Method to get the items placed in the wrong compartment (those found
    // in the both compartments are wrong, but we have no idea what would be
    // the right compartment for that specific item)
    pub fn score(&self) -> (Vec<u8>, u8) {
        let mut commons: Vec<u8> = vec![];

        // We're cloning the common (placed in both compartments) items
        // instead of refering to it later on, so we can just into_iter() it
        // instead of only iter()
        for item in self.one.into_iter() {
            for other in self.two.into_iter() {
                if commons.contains(other) {
                    continue;
                }
                if item == other {
                    commons.push(item.clone());
                    continue;
                }
            }
        }

        // On the other hand, we want to iter() here because we're going to
        // use te commons vector in an outer scope, so don't want to move
        // ownership of its items
        let total = commons.iter()
            .map(|common| {
                if common.is_ascii_lowercase() {
                    // ASCII lowercase starts at 97, but scores are counted
                    // from 1 to 26
                    common - 97 + 1
                } else {
                    // ASCII uppercase starts at 65, but scores are counted
                    // from 27 to 52
                    common - 65 + 27
                }
            })
            .sum();

        return (commons, total);
    }
}

// As we did in the previous puzzle, lets create a trait and bind it to the
// utils::read_lines() function, so we can remove the converstion logic from
// the input file to the puzzle's data from the main function/module.
pub trait Rucksack {
    fn as_rucksack(&mut self) -> Vec<Compartments>;
}

impl Rucksack for io::Lines<BufReader<File>> {
    fn as_rucksack(&mut self) -> Vec<Compartments> {
        let mut cmpts: Vec<Compartments> = vec![];

        // Split each line from input file in two halfs: the first one
        // represents the items in the compartment one of our rucksack and
        // the second half represents the items placed in the compartment
        // two of our rucksack.
        for line_res in self.into_iter() {
            if let Ok(line) = line_res {
                let (first, second) = line.split_at(line.len()/2);

                // We could've called .into() here, but to make sure we
                // understand what's going on we made the conversion
                // ourselves: for both string slices (&str) we first convert
                // to an slice of bytes with .as_bytes(), since we want to
                // operate on each char, than we copy the bytes slice into
                // an owned vector with .to_owned(), which converts any
                // slice &[T] into Vec<T>, than, finally, we can get our raw
                // Box<[u8]> by calling .into_boxed_slice(), which
                // "unsafely" get the Vec<T> pointer length and data and
                // create the boxed [T], ignoring other Vec<> constructs
                // (like capacity).
                //
                // Refs:
                //  https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#impl-ToOwned-for-%5BT%5D
                //  https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1100
                let cmpt = Compartments {
                    one: first.as_bytes().to_owned().into_boxed_slice(),
                    two: second.as_bytes().to_owned().into_boxed_slice(),
                };
                cmpts.push(cmpt);
            }
        }

        return cmpts;
    }
}
