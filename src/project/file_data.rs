use crate::parser::File;

pub struct FileData {
    pub end: u32,
    pub ast: File,
    pub text: String,
    pub name: String,
}
