use ariadne::Source;

use crate::{
    parser::parse_file,
    project::{file_data::FileData, module::ModuleNode},
};

mod file_data;
mod module;

pub struct Project {
    root: ModuleNode,
    files: Vec<FileData>,
    parents: Vec<u32>,
}

impl Project {
    pub fn insert_file(&mut self, text: String, name: String, counter: &mut u32) {
        let ast = parse_file(&text, &name, &Source::from(text.clone()), counter);
        let path = name.split('/').collect::<Vec<&str>>();
        for item in &ast {
            if let Some(name) = item.0.get_name() {
                let id = item.0.get_id().unwrap();
                if item.0.is_parent() {
                    self.parents.push(id);
                }
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

    pub fn get_parent(&self, for_id: u32) -> Option<u32> {
        self.parents.iter().rev().find(|&&id| id < for_id).copied()
    }
}
