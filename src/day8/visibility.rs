use super::tree_map::Tree;

pub trait Visibility {
    fn has_row_visibility(&self, tree_height: u32, x: usize, y: usize) -> bool;
    fn has_collumn_visibility(&self, tree_height: u32, x: usize, y: usize) -> bool;
}

impl Visibility for Vec<Vec<Tree>> {
    fn has_row_visibility(&self, tree_height: u32, x: usize, y: usize) -> bool {
        let row = &self[y];
        row[x + 1..].iter().all(|other| other.height < tree_height)
            || row[..x].iter().all(|other| other.height < tree_height)
    }
    fn has_collumn_visibility(&self, tree_height: u32, x: usize, y: usize) -> bool {
        let row = &transpose(&self)[y];
        row[x + 1..].iter().all(|other| other.height < tree_height)
            || row[..x].iter().all(|other| other.height < tree_height)
    }
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
