#[derive(Clone, Copy)]
pub struct Elf {
    pub id: usize, // we don't really need id, that's just out of curiosity
    pub calories: i32,
}

pub trait RankedElves {
    fn elves_rank_insert(&mut self, value: Elf);
    fn elves_rank_total(&self) -> i32;
}

impl RankedElves for Vec<Elf> {
    fn elves_rank_insert(&mut self, elf: Elf) {
        let v_len = self.len();

        for i in 0..v_len {
            if elf.calories > self[i].calories {
                self.insert(i, elf);
                self.pop();
                return;
            }
        }
    }

    fn elves_rank_total(&self) -> i32 {
        let mut total: i32 = 0;
        self.iter().for_each(|elf| total += elf.calories);
        return total;
    }
}

