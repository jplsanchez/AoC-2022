pub mod command;
pub mod directory;

use command::Command;
use directory::Directory;

// const PATH: &str = "src/day7/sample.txt";
const PATH: &str = "src/day7/input.txt";

fn setup() -> Directory {
    let history = crate::utils::read_to_vector(PATH);
    let mut cwd = vec!["/".to_string()];
    let mut root = Directory::new("/".to_string());

    for line in history {
        let command = Command::from(line);
        command.execute(&mut cwd, &mut root);
    }

    root.update_size();

    root
}

pub fn run_part_1() {
    let root = setup();
    let result = root.find_size_less_than(100000);
    println!(
        "Files less than 1000000 bytes sum to: {:?}",
        result.iter().map(|(_, size)| size).sum::<u64>()
    )
}

pub fn run_part_2() {
    const TOTAL_SPACE: u64 = 70_000_000;
    const FREE_SPACE_NEEDED: u64 = 30_000_000;

    let root = setup();
    let needed_to_free = FREE_SPACE_NEEDED - (TOTAL_SPACE - root.size);

    let mut list = root.list_all_dir_size();
    list.sort_by(|a, b| a.1.cmp(&b.1));

    for dir in list.iter() {
        if dir.1 >= needed_to_free {
            println!(
                "The directory to delete is: '{}' with size {}",
                dir.0, dir.1
            );
            break;
        }
    }
}
