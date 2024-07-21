use ariadne::Source;

use crate::parser::{parse_file, File};

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

    pub fn insert(&mut self, path: &[&str], id: u32, name: &str) {
        if path.is_empty() {
            self.children.push(ModuleNode::item(name.to_string(), id));
            return;
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

    pub fn get_id(&self, path: &[String]) -> Option<u32> {
        if path.is_empty() {
            return Some(self.id);
        } else {
            for child in &self.children {
                if child.name == path[0] {
                    return child.get_id(&path[1..]);
                }
            }
            None
        }
    }
}

pub struct Project {
    root: ModuleNode,
    files: Vec<FileData>,
}

pub struct FileData {
    end: u32,
    ast: File,
    text: String,
    name: String,
}

impl Project {
    pub fn insert_file(&mut self, text: String, name: String, counter: &mut u32) {
        let ast = parse_file(&text, &name, &Source::from(text.clone()), counter);
        let path = name.split('/').collect::<Vec<&str>>();
        for item in &ast {
            if let Some(name) = item.0.get_name() {
                let id = item.0.get_id().unwrap();
                self.root.insert(&path, id, name);
            }
        }
        let file_data = FileData {
            end: *counter,
            ast,
            text,
            name,
        };
        self.files.push(file_data);
    }

    pub fn get_file(&self, for_id: u32) -> Option<&FileData> {
        self.files.iter().find(|f| f.end > for_id)
    }
}
