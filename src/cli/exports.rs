use ptree::print_tree;

use crate::fs::project::Project;

pub fn exports() {
    let mut project = Project::init_pwd();
    project.build_impls();
    project.check();
    let tree = project.build_tree();
    print_tree(&tree).unwrap();
}
