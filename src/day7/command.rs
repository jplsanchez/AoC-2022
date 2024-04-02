use super::directory::Directory;

#[derive(Debug)]
pub enum Command {
    ChangeDirectory { target: String },
    ListFiles,
    AddDirectory { name: String },
    AddFile { name: String, size: u64 },
}

impl Command {
    pub fn execute(&self, cwd: &mut Vec<String>, root: &mut Directory) {
        match self {
            Command::ChangeDirectory { target } => match target.as_str() {
                "/" => {
                    cwd.clear();
                    cwd.push("/".to_string());
                }
                ".." => {
                    cwd.pop();
                }
                _ => cwd.push(target.to_string()),
            },
            Command::ListFiles => {}
            Command::AddDirectory { name } => {
                let current = root
                    .find_cwd_mut(cwd)
                    .expect(format!("Could not find current directory {:?}", cwd).as_str());
                current.add_directory(name.to_string());
            }
            Command::AddFile { name, size } => {
                let current = root
                    .find_cwd_mut(cwd)
                    .expect(format!("Could not find current directory {:?}", cwd).as_str());
                current.add_file(name.to_string(), *size);
            }
        }
    }
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => Command::ChangeDirectory {
                    target: parts[2].to_string(),
                },
                "ls" => Command::ListFiles,
                other => panic!("Unknown command {}", other),
            },
            "dir" => Command::AddDirectory {
                name: parts[1..].join(" ").to_string(),
            },
            number if number.parse::<u64>().is_ok() => Command::AddFile {
                name: parts[1..].join(" ").to_string(),
                size: number.parse().unwrap(),
            },
            other => panic!("Unknown command {}", other),
        }
    }
}
