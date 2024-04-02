pub struct Directory {
    pub name: String,
    pub size: u64,
    children: Vec<Directory>,
    files: Vec<(String, u64)>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            size: 0,
            children: vec![],
            files: vec![],
        }
    }

    pub fn add_directory(&mut self, name: String) {
        self.children.push(Directory::new(name));
    }

    pub fn add_file(&mut self, name: String, size: u64) {
        self.files.push((name, size));
    }

    pub fn find_child_mut(&mut self, name: &str) -> Option<&mut Directory> {
        self.children.iter_mut().find(|child| child.name == name)
    }

    pub fn find_cwd_mut(&mut self, cwd: &Vec<String>) -> Option<&mut Directory> {
        if cwd.last().unwrap() == &self.name {
            return Some(self);
        }

        let mut current = self;

        for dir in cwd.iter().skip(1) {
            current = current.find_child_mut(dir)?;
        }

        Some(current)
    }

    pub fn update_size(&mut self) {
        self.size = self.files.iter().map(|(_, size)| size).sum::<u64>()
            + self
                .children
                .iter_mut()
                .map(|child| {
                    child.update_size();
                    child.size
                })
                .sum::<u64>();
    }

    pub fn find_size_less_than(&self, max_size: u64) -> Vec<(String, u64)> {
        let mut result = vec![];

        if self.size <= max_size {
            result.push((self.name.clone(), self.size));
        }

        for child in &self.children {
            result.append(&mut child.find_size_less_than(max_size));
        }

        result
    }

    pub fn list_all_dir_size(&self) -> Vec<(String, u64)> {
        let mut result = vec![];

        result.push((self.name.clone(), self.size));

        for child in &self.children {
            result.append(&mut child.list_all_dir_size());
        }

        result
    }
}
