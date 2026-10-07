use crate::{command::CommandSummary, visitor::DirVisitor};
use clap::{Arg, ArgAction};

pub fn tree_argument() -> Arg {
    Arg::new("tree")
        .short('t')
        .long("tree")
        .action(ArgAction::SetTrue)
        .required(false)
        .help("Tree representation enabeling.")
}

pub const BRANCH_CONNECTOR: &str = "├──";
pub const LEAF_CONNECTOR: &str = "└──";
pub const VERTICAL_CONNECTOR: &str = "│   ";
pub const EMPTY_CONNECTOR: &str = "    ";

pub fn display_tree(sum: CommandSummary) -> String {
    let mut visitor = DirVisitor::from(sum);
    visitor.visit();
    visitor.get_result_string()
}

pub fn get_tree(layers: &mut Vec<usize>, connector: &str) -> Option<String> {
    let mut s = String::new();
    if let [.., last] = layers.as_slice() {
        for i in 0..*last {
            if layers.contains(&i) {
                s.push_str(VERTICAL_CONNECTOR)
            } else {
                s.push_str(EMPTY_CONNECTOR)
            }
        }
    } else {
        s.push_str(EMPTY_CONNECTOR);
    }
    s.push_str(connector);
    Some(s)
}
