use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError},
        state::CheckState,
    },
    util::Spanned,
};

pub struct Node {
    name: String,
    id: u32,
    children: Vec<Node>,
}

impl Node {
    pub fn module(name: String) -> Node {
        Node {
            name,
            id: 0,
            children: Vec::new(),
        }
    }

    pub fn item(name: String, id: u32) -> Node {
        Node {
            name,
            id,
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, path: &[String], id: u32, name: &str) {
        if path.is_empty() {
            self.children.push(Node::item(name.to_string(), id));
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
                let mut child = Node::module(path[0].to_string());
                child.insert(&path[1..], id, name);
                self.children.push(child);
            }
        }
    }

    pub fn get_with_error(&self, path: &[Spanned<String>], state: &mut CheckState) -> Option<u32> {
        if path.is_empty() {
            Some(self.id)
        } else if let Some(child) = self.children.iter().find(|c| c.name == path[0].0) {
            return child.get_with_error(&path[1..], state);
        } else {
            state.error(CheckError::Unresolved(Unresolved {
                name: path[0].clone(),
                file: state.file_data.end,
            }));
            None
        }
    }

    #[cfg(test)]
    pub fn get_path(&self, path: &[&str]) -> Option<u32> {
        let next = path.first();
        if next.is_none() {
            Some(self.id)
        } else if let Some(child) = self.children.iter().find(|c| &c.name == next.unwrap()) {
            return child.get_path(&path[1..]);
        } else {
            None
        }
    }

    pub fn get_without_error(&self, path: &[Spanned<String>]) -> Option<u32> {
        let next = path.first();
        if next.is_none() {
            Some(self.id)
        } else if let Some(child) = self.children.iter().find(|c| c.name == next.unwrap().0) {
            return child.get_without_error(&path[1..]);
        } else {
            None
        }
    }

    pub fn get_module(&self, path: &[String]) -> Option<&Node> {
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
    use super::Node;

    #[test]
    fn test_new_module() {
        let module = Node::module("my_module".to_string());
        assert!(module.children.is_empty(), "Children should be empty");
        assert_eq!(module.name, "my_module", "Name should be 'my_module'");
        assert_eq!(module.id, 0, "ID should be 0");
    }

    #[test]
    fn test_new_item() {
        let item = Node::item("my_item".to_string(), 42);
        assert!(item.children.is_empty(), "Children should be empty");
        assert_eq!(item.name, "my_item", "Name should be 'my_item'");
        assert_eq!(item.id, 42, "ID should be 42");
    }

    #[test]
    fn insert_empty_path() {
        let mut module = Node::module("my_module".to_string());
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
        let mut module = Node::module("my_module".to_string());
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
        let mut module = Node::module("my_module".to_string());
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
