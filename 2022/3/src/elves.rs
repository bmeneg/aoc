use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::rc::{Rc, Weak};

// Each rucksack has two different compartments
#[derive(Debug)]
pub struct Rucksack {
    pub compartments: [Vec<u8>; 2],
}

// Use the iterator std::io::Lines<> as actual rucksack compartments, no
// need to move this logic to somewhere else later on, we can make it right
// at the input file reading.
pub trait AsRucksack {
    fn as_rucksack(self) -> Vec<Rucksack>;
}

impl AsRucksack for Lines<BufReader<File>> {
    fn as_rucksack(self) -> Vec<Rucksack> {
        let mut rucksacks: Vec<Rucksack> = vec![];

        let lines: Vec<String> = self.map(|l| l.unwrap()).collect();
        for line in lines {
            // each half of the line is one compartment
            let (fh, sh) = line.split_at(line.len() / 2);
            rucksacks.push(Rucksack {
                compartments: [fh.as_bytes().to_owned(), sh.as_bytes().to_owned()],
            });
        }

        return rucksacks;
    }
}

// Each Elf has a single Rucksack and is part of a single group; an elf
// cannot have no group.
#[derive(Debug)]
pub struct Elf {
    pub rucksack: Rucksack,
    // We want to find other elfs we have in the same group
    group: Option<Weak<RefCell<Group>>>,
}

impl Elf {
    pub fn new(rs: Rucksack) -> Self {
        Self {
            rucksack: rs,
            group: None,
        }
    }

    pub fn from_rucksacks(rucksacks: Vec<Rucksack>) -> Vec<Self> {
        let mut elves: Vec<Self> = vec![];
        for rs in rucksacks {
            elves.push(Self::new(rs));
        }
        return elves;
    }
}

// A group is composed of at max 3 elves, but at the same time it can have
// no elves at all. The badge is defined at runtime based on what each
// group's elf has on their rucksack.
#[derive(Debug)]
pub struct Group {
    elves: [Option<Elf>; 3],
    pub badge: Option<u8>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            elves: [None, None, None],
            badge: None,
        }
    }

    pub fn from_elves(elves: Vec<Elf>) -> Vec<Rc<RefCell<Group>>> {
        let mut groups = vec![];
        let mut group = Rc::new(RefCell::new(Group::new()));

        for mut elf in elves {
            // Check in which index we can add the next elf; remember, we're
            // using and array of Elfs instead of a Vec<> that we can expand
            // needed
            let mut last_idx: Option<usize> = group.borrow().last();
            if last_idx.is_some() {
                elf.group = Some(Rc::downgrade(&group));
                group.borrow_mut().elves[last_idx.unwrap()] = Some(elf);

                // Finish previous group if that's the last free slot
                last_idx = group.borrow().last();
                if last_idx.is_none() {
                    group.borrow_mut().set_badge();
                    groups.push(Rc::clone(&group));
                }
                continue;
            }

            // Create the new one with the leftover elf
            group = Rc::new(RefCell::new(Group::new()));
            elf.group = Some(Rc::downgrade(&group));
            group.borrow_mut().elves[0] = Some(elf);
        }

        return groups;
    }

    // Get the last "empty" (None) slot in the elves array
    fn last(&self) -> Option<usize> {
        for (i, elf) in self.elves.iter().enumerate() {
            if elf.is_none() {
                return Some(i);
            }
        }
        return None;
    }

    // Set group's badge based on the common item
    fn set_badge(&mut self) {
        let mut items_map: HashMap<u8, u8> = HashMap::new();

        for elf_opt in self.elves.iter() {
            let elf = elf_opt.as_ref().unwrap();
            // Join both rucksack's compartments
            let mut items = elf.rucksack.compartments[0].to_vec();
            items.extend(elf.rucksack.compartments[1].to_vec());
            // Use a "gone list" to uniquely count the items
            let mut gone_items: Vec<u8> = vec![];

            // Traverse the list of items counting (starting on 0) them
            // uniquely into a HashMap. Also, since our item list is a copy
            // we can move the value here instead of creating a ref to it.
            for item in items {
                items_map
                    .entry(item)
                    .and_modify(|v| {
                        if !gone_items.contains(&item) {
                            *v += 1;
                        }
                    })
                    .or_insert(0);
                gone_items.push(item);
            }
        }

        // Since we counted unique items, we're guaranteed that common items
        // have the counter set to the number of elves minus one.
        for (k, v) in items_map {
            if v == (self.elves.len() as u8 - 1) {
                self.badge = Some(k);
            }
        }
    }
}
