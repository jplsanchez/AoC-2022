#![allow(dead_code)]

use super::visibility::Visibility;

pub struct TreeMap {
    map: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

#[derive(Clone)]
pub struct Tree {
    pub height: u32,
    is_visible: bool,
}

impl TreeMap {
    pub fn new(data: Vec<String>) -> Self {
        let mut map: Vec<Vec<Tree>> = vec![];

        for line in data {
            let mut row: Vec<Tree> = vec![];
            for c in line.chars() {
                row.push(Tree::new(c.to_digit(10).unwrap()))
            }
            map.push(row);
        }

        Self {
            map: map.clone(),
            width: map[0].len(),
            height: map.len(),
        }
        .set_visibility()
    }

    fn set_visibility(mut self) -> Self {
        let map = self.map.clone();
        self.map.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, tree)| {
                tree.is_visible = match (x, y) {
                    (0, _) => true,
                    (x, _) if x == self.width - 1 => true,
                    (_, 0) => true,
                    (_, y) if y == self.height - 1 => true,
                    (x, y) => {
                        map.has_row_visibility(tree.height, x, y)
                            || map.has_collumn_visibility(tree.height, x, y)
                    }
                }
            })
        });
        self
    }

    pub fn count_visible_trees(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|tree| tree.is_visible).count())
            .sum()
    }

    pub fn print_visibility(&self) {
        for row in &self.map {
            for tree in row {
                print!("{}", tree.is_visible as u8);
            }
            println!();
        }
    }

    pub fn print_height(&self) {
        for row in &self.map {
            for tree in row {
                print!("{}", tree.height);
            }
            println!();
        }
    }
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {
            height,
            is_visible: false,
        }
    }
}
