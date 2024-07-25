use crate::project::name::QualifiedName;

pub fn path_from_filename(filename: &str) -> QualifiedName {
    filename
        .strip_suffix(".gib")
        .unwrap()
        .split('/')
        .map(str::to_string)
        .collect::<QualifiedName>()
}
