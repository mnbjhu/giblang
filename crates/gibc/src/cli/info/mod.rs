mod file_tree;
mod module_tree;

#[derive(Debug, clap::Subcommand)]
pub enum InfoCommand {
    /// Show the module tree
    ModuleTree,
    /// Show the file tree
    FileTree,
}

impl InfoCommand {
    pub fn run(&self) {
        match self {
            InfoCommand::ModuleTree => module_tree::module_tree(),
            InfoCommand::FileTree => file_tree::file_tree(),
        }
    }
}
