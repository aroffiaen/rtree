pub mod command;
pub mod fileonly;
pub mod size;
pub mod sorter;
pub mod stats;
pub mod tree;
pub mod visitor;
pub mod walking_directories;

use crate::{
    command::{get_command, CommandSummaryBuilder, CURRENT_DIRECTORY},
    sorter::Sortby,
};
use std::path::PathBuf;

fn main() {
    let matches = get_command().get_matches();

    let command = CommandSummaryBuilder::new()
        .dir(
            matches
                .get_one::<PathBuf>("dir")
                .or(Some(&PathBuf::from(CURRENT_DIRECTORY)))
                .cloned()
                .unwrap(),
        )
        .max(matches.get_one::<usize>("max").cloned())
        .sort(matches.get_one::<Sortby>("sort").cloned())
        .stats(match matches.get_one("stats") {
            Some(&s) => s,
            None => false,
        })
        .tree(match matches.get_one("tree") {
            Some(&t) => t,
            None => false,
        })
        .size(match matches.get_one("size") {
            Some(&s) => s,
            None => false,
        })
        .file(match matches.get_one("file") {
            Some(&f) => f,
            None => false,
        })
        .all(match matches.get_one("all") {
            Some(&a) => a,
            None => false,
        })
        .build();

    command.exec();
}
