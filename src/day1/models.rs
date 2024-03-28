pub struct CaloriesRank {
    pub rank: [i32; 3],
}

impl CaloriesRank {
    pub fn try_add_to_rank(&mut self, calories: i32) {
        for i in 0..self.rank.len() {
            if calories <= self.rank[i] {
                break;
            }
            match i {
                0 => self.rank[i] = calories,
                _ => {
                    self.rank[i - 1] = self.rank[i];
                    self.rank[i] = calories;
                }
            }
        }
    }

    pub fn sum_rank(&self) -> i32 {
        self.rank.iter().sum()
    }
}

pub struct Elf {
    pub calories: i32,
}

impl Elf {
    pub fn new() -> Elf {
        Elf { calories: 0 }
    }
}
