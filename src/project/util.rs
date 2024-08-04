use crate::project::name::QualifiedName;

pub fn path_from_filename(filename: &str) -> QualifiedName {
    filename
        .strip_suffix(".gib")
        .unwrap()
        .split('/')
        .map(str::to_string)
        .collect::<QualifiedName>()
}

#[cfg(test)]
mod tests {
    use crate::project::util::path_from_filename;

    #[test]
    fn test_get_path() {
        assert_eq!(path_from_filename("name.gib"), vec!["name"]);
        assert_eq!(
            path_from_filename("module/name.gib"),
            vec!["module", "name"]
        );
    }
}
