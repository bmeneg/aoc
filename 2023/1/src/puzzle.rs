use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Lines};

pub trait AsCallibValues {
    fn as_callibration(self) -> Vec<u32>;
}

impl AsCallibValues for Lines<BufReader<File>> {
    fn as_callibration(self) -> Vec<u32> {
        let digit_str = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut values: Vec<u32> = vec![];
        let lines: Vec<String> = self.map(|l| l.unwrap()).collect();

        for line in lines {
            // Binary Tree Map to store an ordered map based on the keys
            let mut value_digits: BTreeMap<usize, usize> = BTreeMap::new();

            // first, check for the written digits
            for (i, name) in digit_str.iter().enumerate() {
                let mut line_copy = &line.clone()[..];
                // pivot keep the original index
                let mut line_idx;

                // repeat the find operation looking for the same number
                // name in the remaining of the line
                loop {
                    let local_idx = line_copy.find(name);
                    if let Some(idx) = local_idx {
                        line_idx = idx + (line.len() - line_copy.len());
                        value_digits.insert(line_idx, i + 1);
                        if line_idx + name.len() > line.len() {
                            break;
                        }
                        line_copy = &line_copy[idx + name.len()..];
                    } else {
                        break;
                    }
                }
            }

            // then, check for the number digits
            for (i, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    value_digits.insert(i, c.to_digit(10).unwrap() as usize);
                }
            }

            // IDK if all lines has a valid digit
            if value_digits.is_empty() {
                continue;
            }

            // From the previous conditional we know we have valid digits
            // within value_digits, thus it's fine to just unwrap() and
            // get() them
            let value_str: String;
            if value_digits.len() == 1 {
                let value = value_digits.values().next().unwrap();
                value_str = format!("{}{}", value, value);
            } else {
                let first = *value_digits.first_entry().unwrap().get();
                let last = *value_digits.last_entry().unwrap().get();
                value_str = format!("{}{}", first, last);
            }
            println!("{:?} > {value_str}", value_digits);
            values.push(value_str.parse::<u32>().unwrap());
        }

        values
    }
}
