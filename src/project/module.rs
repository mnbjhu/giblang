use crate::util::Spanned;

use super::file_data::FileData;

pub struct ModuleNode {
    name: String,
    id: u32,
    children: Vec<ModuleNode>,
}

impl ModuleNode {
    pub fn module(name: String) -> ModuleNode {
        ModuleNode {
            name,
            id: 0,
            children: Vec::new(),
        }
    }

    pub fn item(name: String, id: u32) -> ModuleNode {
        ModuleNode {
            name,
            id,
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, path: &[String], id: u32, name: &str) {
        if path.is_empty() {
            self.children.push(ModuleNode::item(name.to_string(), id));
        } else {
            let mut found = false;
            for child in &mut self.children {
                if child.name == path[0] {
                    child.insert(&path[1..], id, name);
                    found = true;
                    break;
                }
            }
            if !found {
                let mut child = ModuleNode::module(path[0].to_string());
                child.insert(&path[1..], id, name);
                self.children.push(child);
            }
        }
    }

    // TODO: Delete if not needed
    #[allow(dead_code)]
    pub fn get_id(&self, path: &[String]) -> Option<u32> {
        if path.is_empty() {
            Some(self.id)
        } else {
            for child in &self.children {
                if child.name == path[0] {
                    return child.get_id(&path[1..]);
                }
            }
            None
        }
    }

    pub fn get_with_error(&self, path: &[Spanned<String>], file: &FileData) -> Option<u32> {
        if path.is_empty() {
            Some(self.id)
        } else if let Some(child) = self.children.iter().find(|c| c.name == path[0].0) {
            return child.get_with_error(&path[1..], file);
        } else {
            file.error(&format!("Import '{}' not found", path[0].0), path[0].1);
            None
        }
    }

    // TODO: Delete if not needed
    #[allow(dead_code)]
    pub fn get_without_error(&self, path: &[Spanned<String>]) -> Option<u32> {
        if path.is_empty() {
            Some(self.id)
        } else if let Some(child) = self.children.iter().find(|c| c.name == path[0].0) {
            return child.get_without_error(&path[1..]);
        } else {
            None
        }
    }

    pub fn get_module(&self, path: &[String]) -> Option<&ModuleNode> {
        if path.is_empty() {
            Some(self)
        } else if let Some(child) = self.children.iter().find(|c| c.name == path[0]) {
            return child.get_module(&path[1..]);
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::build::build;

    #[test]
    fn run() {
        build()
    }
}
