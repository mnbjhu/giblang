use ptree::print_tree;

use crate::project::Project;

pub fn exports() {
    let mut project = Project::init_pwd();
    // project.check();
    // let tree = project.build_tree();
    // print_tree(&tree).unwrap();
}
