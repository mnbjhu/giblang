use ptree::print_tree;

use crate::fs::project::Project;

pub fn exports() {
    let project = Project::init_pwd();
    let tree = project.build_tree();
    print_tree(&tree).unwrap();
}
