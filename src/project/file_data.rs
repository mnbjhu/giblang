use crate::{parser::File, project::util::path_from_filename};

pub struct FileData {
    pub end: u32,
    pub ast: File,
    pub text: String,
    pub name: String,
}

impl FileData {
    #[must_use]
    pub fn get_path(&self) -> Vec<String> {
        path_from_filename(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::project::file_data::FileData;

    #[test]
    fn test_get_path() {
        let file_data = FileData {
            end: 0,
            ast: Vec::default(),
            text: "text".to_string(),
            name: "name.gib".to_string(),
        };

        assert_eq!(file_data.get_path(), vec!["name"]);
    }
}
