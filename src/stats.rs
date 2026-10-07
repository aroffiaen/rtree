use clap::{Arg, ArgAction};
use walkdir::DirEntry;

pub fn stats_argument() -> Arg {
    Arg::new("stats")
        .long("stats")
        .action(ArgAction::SetTrue)
        .required(false)
        .help("Get the statistics about files in the directory.")
}

#[derive(Clone, Default)]
pub struct FileStats {
    file_count: usize,
}

impl FileStats {
    pub fn update(&mut self, entry: &DirEntry) {
        if entry.file_type().is_file() {
            self.file_count += 1;
        }
    }
}
