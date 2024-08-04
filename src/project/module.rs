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
    use super::ModuleNode;

    #[test]
    fn test_new_module() {
        let module = ModuleNode::module("my_module".to_string());
        assert!(module.children.is_empty(), "Children should be empty");
        assert_eq!(module.name, "my_module", "Name should be 'my_module'");
        assert_eq!(module.id, 0, "ID should be 0");
    }

    #[test]
    fn test_new_item() {
        let item = ModuleNode::item("my_item".to_string(), 42);
        assert!(item.children.is_empty(), "Children should be empty");
        assert_eq!(item.name, "my_item", "Name should be 'my_item'");
        assert_eq!(item.id, 42, "ID should be 42");
    }

    #[test]
    fn insert_empty_path() {
        let mut module = ModuleNode::module("my_module".to_string());
        module.insert(&[], 42, "my_item");
        assert_eq!(module.children.len(), 1, "Children should have 1 item");
        assert_eq!(
            module.children[0].name, "my_item",
            "Name should be 'my_item'"
        );
        assert_eq!(module.children[0].id, 42, "ID should be 42");
    }

    #[test]
    fn insert_path_size_one() {
        let mut module = ModuleNode::module("my_module".to_string());
        module.insert(&["sub_module".to_string()], 42, "my_item");
        assert_eq!(module.children.len(), 1, "Children should have 1 item");
        let child = &module.children[0];
        assert_eq!(child.name, "sub_module", "Name should be 'sub_module'");
        assert_eq!(child.id, 0, "ID should be 0");
        assert_eq!(child.children.len(), 1, "Children should have 1 item");
        let item = &child.children[0];
        assert_eq!(item.name, "my_item", "Name should be 'my_item'");
        assert_eq!(item.id, 42, "ID should be 42");
        assert!(item.children.is_empty(), "Children should be empty");
    }

    #[test]
    fn insert_into_existing_child() {
        let mut module = ModuleNode::module("my_module".to_string());
        module.insert(
            &[
                "first_sub_module".to_string(),
                "second_sub_module".to_string(),
            ],
            42,
            "my_item",
        );
        module.insert(
            &[
                "first_sub_module".to_string(),
                "second_sub_module".to_string(),
            ],
            69,
            "other_item",
        );
        assert_eq!(module.children.len(), 1, "Children should have 1 item");

        let child = &module.children[0];
        assert_eq!(
            child.name, "first_sub_module",
            "Name should be 'first_sub_module'"
        );
        assert_eq!(child.id, 0, "ID should be 0");
        assert_eq!(child.children.len(), 1, "Children should have 2 items");

        let child = &child.children[0];
        assert_eq!(
            child.name, "second_sub_module",
            "Name should be 'second_sub_module'"
        );
        assert_eq!(child.id, 0, "ID should be 0");
        assert_eq!(child.children.len(), 2, "Children should have 2 items");

        let first_item = &child.children[0];
        assert_eq!(first_item.name, "my_item", "Name should be 'my_item'");
        assert_eq!(first_item.id, 42, "ID should be 42");
        assert!(first_item.children.is_empty(), "Children should be empty");

        let second_item = &child.children[1];
        assert_eq!(
            second_item.name, "other_item",
            "Name should be 'other_item'"
        );
        assert_eq!(second_item.id, 69, "ID should be 69");
        assert!(second_item.children.is_empty(), "Children should be empty");
    }
}
