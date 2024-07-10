use ptree::{print_tree, TreeBuilder};

use crate::fs::project::Project;

pub fn exports() {
    let project = Project::init_pwd();
    let mut builder = TreeBuilder::new("/".to_string());
    project.exports.build_tree(&mut builder, "/".to_string());
    print_tree(&builder.build()).unwrap();
}
