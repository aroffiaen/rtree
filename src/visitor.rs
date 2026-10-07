use crate::{
    command::CommandSummary,
    size,
    sorter::Sortby,
    stats::FileStats,
    tree::{self, BRANCH_CONNECTOR, LEAF_CONNECTOR},
    walking_directories,
};
use std::path::PathBuf;
use walkdir::DirEntry;

pub struct DirVisitor {
    all: bool,
    tree: bool,
    size: bool,
    dir: PathBuf,
    fileonly: bool,
    symlinks: bool,
    layers: Vec<usize>,
    sorter: Option<Sortby>,
    depth_from_root: usize,
    max_walking_depth: usize,
    max_recursion_depth: usize,
    stats: FileStats,
    result_string: String,
}

impl Default for DirVisitor {
    fn default() -> Self {
        Self {
            all: true,
            tree: false,
            size: false,
            sorter: None,
            layers: vec![],
            fileonly: false,
            symlinks: false,
            depth_from_root: 0,
            dir: PathBuf::new(),
            max_walking_depth: usize::MAX,
            max_recursion_depth: usize::MAX,
            stats: FileStats::default(),
            result_string: String::new(),
        }
    }
}

impl From<CommandSummary> for DirVisitor {
    fn from(value: CommandSummary) -> Self {
        let tree = value.get_tree();
        let max_recursion_depth = value.get_max().unwrap_or(usize::MAX);
        let max_walking_depth = match tree {
            true => 1,
            false => max_recursion_depth,
        };

        Self {
            tree,
            max_walking_depth,
            max_recursion_depth,
            all: value.get_all(),
            size: value.get_size(),
            sorter: value.get_sort(),
            fileonly: value.get_file(),
            symlinks: value.get_symlinks(),
            dir: value.get_dir().to_path_buf(),
            ..Default::default()
        }
    }
}

impl DirVisitor {
    pub fn get_result_string(self) -> String {
        self.result_string
    }

    pub fn get_dir(&self) -> PathBuf {
        self.dir.to_owned()
    }

    pub fn get_sorter(&self) -> Option<Sortby> {
        self.sorter
    }

    pub fn get_max_depth(&self) -> usize {
        self.max_walking_depth
    }

    pub fn allow_symlinks(&self) -> bool {
        self.symlinks
    }

    pub fn is_all(&self) -> bool {
        self.all
    }

    pub fn is_fileonly(&self) -> bool {
        self.fileonly
    }

    fn push_depth(&mut self) {
        self.layers.push(self.depth_from_root);
        if !self.layers.is_sorted() {
            panic!("Layers is not sorted!");
        }
    }

    fn pop_depth(&mut self) {
        if self.layers.last() == Some(&self.depth_from_root) {
            self.layers.pop();
        }
    }

    fn reached_recursion_limit(&self) -> bool {
        self.max_recursion_depth <= self.depth_from_root
    }

    fn entry_to_string(&self, entry: &DirEntry, tree: Option<String>) -> String {
        let path = entry.path();
        let path_end = match path.parent() {
            Some(p) => path.strip_prefix(p).iter().filter_map(Some).collect(),
            _ => path.to_owned(),
        };

        format!(
            "{:>width$} {} {}\n",
            match size::match_entry_size(entry, self.size) {
                Some(size) => size,
                None => "".to_string(),
            },
            tree.as_deref().unwrap_or(""),
            match self.tree {
                true => path_end.display(),
                false => path.display(),
            },
            width = if self.size { 5 } else { 1 },
        )
    }

    fn entry_processing(&mut self, entry: &DirEntry, connector: &str) {
        let saved_current_dir = self.dir.to_owned();
        let tree = match self.tree {
            true => tree::get_tree(&mut self.layers, connector),
            false => None,
        };
        let path = entry.path().to_owned();

        self.stats.update(entry);
        self.result_string
            .push_str(&self.entry_to_string(entry, tree));

        if self.tree && entry.file_type().is_dir() && !self.reached_recursion_limit() {
            self.dir = path;
            self.depth_from_root += 1;
            self.visit();
            self.dir = saved_current_dir;
            self.depth_from_root -= 1;
        }
    }

    pub fn visit(&mut self) {
        self.push_depth();
        if self.reached_recursion_limit() {
            self.pop_depth();
            return;
        }

        let (walk, walk_size) = walking_directories::get_dir_entries(self);
        if walk_size != 0 {
            for entry in walk[..walk_size - 1].iter() {
                self.entry_processing(entry, BRANCH_CONNECTOR);
            }
            self.entry_processing(&walk[walk_size - 1], LEAF_CONNECTOR);
        }

        self.pop_depth();
    }
}
