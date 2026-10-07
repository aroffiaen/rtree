use crate::{sorter::Sortby, visitor::DirVisitor};
use clap::{builder::PathBufValueParser, value_parser, Arg, ArgAction, Command};
use std::path::{Path, PathBuf};

pub const CURRENT_DIRECTORY: &str = ".";

pub struct CommandSummary {
    all: bool,
    tree: bool,
    size: bool,
    file: bool,
    stats: bool,
    dir: PathBuf,
    symlinks: bool,
    max: Option<usize>,
    sort: Option<Sortby>,
}

pub struct CommandSummaryBuilder {
    all: bool,
    tree: bool,
    size: bool,
    file: bool,
    stats: bool,
    dir: PathBuf,
    symlinks: bool,
    max: Option<usize>,
    sort: Option<Sortby>,
}

impl CommandSummaryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dir(self, dir: PathBuf) -> Self {
        Self { dir, ..self }
    }

    pub fn tree(self, tree: bool) -> Self {
        Self { tree, ..self }
    }

    pub fn size(self, size: bool) -> Self {
        Self { size, ..self }
    }

    pub fn max(self, max: Option<usize>) -> Self {
        Self { max, ..self }
    }

    pub fn sort(self, sort: Option<Sortby>) -> Self {
        Self { sort, ..self }
    }

    pub fn file(self, file: bool) -> Self {
        Self { file, ..self }
    }

    pub fn symlinks(self, symlinks: bool) -> Self {
        Self { symlinks, ..self }
    }

    pub fn all(self, all: bool) -> Self {
        Self { all, ..self }
    }

    pub fn stats(self, stats: bool) -> Self {
        Self { stats, ..self }
    }

    pub fn build(self) -> CommandSummary {
        CommandSummary {
            all: self.all,
            tree: self.tree,
            size: self.size,
            file: self.file,
            stats: self.stats,
            dir: self.dir,
            symlinks: self.symlinks,
            max: self.max,
            sort: self.sort,
        }
    }
}

impl CommandSummary {
    pub fn get_dir(&self) -> &Path {
        self.dir.as_path()
    }

    pub fn get_tree(&self) -> bool {
        self.tree
    }

    pub fn get_size(&self) -> bool {
        self.size
    }

    pub fn get_max(&self) -> Option<usize> {
        self.max
    }

    pub fn get_sort(&self) -> Option<Sortby> {
        self.sort
    }

    pub fn get_file(&self) -> bool {
        self.file
    }

    pub fn get_symlinks(&self) -> bool {
        self.symlinks
    }

    pub fn get_all(&self) -> bool {
        self.all
    }

    pub fn get_stats(&self) -> bool {
        self.stats
    }

    pub fn exec(self) {
        let mut visitor = DirVisitor::from(self);
        visitor.visit();
        print!("{}", visitor.get_result_string());
    }
}

impl Default for CommandSummaryBuilder {
    fn default() -> Self {
        Self {
            max: None,
            sort: None,
            all: false,
            tree: false,
            size: false,
            file: false,
            stats: false,
            symlinks: false,
            dir: PathBuf::from(CURRENT_DIRECTORY),
        }
    }
}

pub fn get_command() -> Command {
    Command::new("rtree")
        .version("0.1.0")
        .about("Display files and directories recursively.")
        .arg(
            Arg::new("dir")
                .short('d')
                .long("directory")
                .default_value(".")
                .value_parser(PathBufValueParser::default())
                .required(false)
                .help("Starting directory."),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Enable display of all hidden folders and files."),
        )
        .arg(crate::size::size_argument())
        .arg(crate::fileonly::fileonly_argument())
        .arg(crate::stats::stats_argument())
        .arg(crate::sorter::sorter_argument())
        .arg(
            Arg::new("max")
                .short('m')
                .long("max-depth")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Maximum recursion depth"),
        )
        .arg(crate::tree::tree_argument())
}
