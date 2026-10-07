use crate::{sorter::Sortby, visitor::DirVisitor};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn get_walkdir<P: AsRef<Path>>(
    root: P,
    symlinks: bool,
    sortby: Option<Sortby>,
    depth: usize,
) -> WalkDir {
    let walk = WalkDir::new(root).follow_links(symlinks).max_depth(depth);
    match sortby {
        Some(Sortby::DirFirst) => walk.sort_by_key(|a| a.path().is_file()),
        Some(Sortby::DirLast) => walk.sort_by_key(|a| a.path().is_dir()),
        Some(Sortby::Size) => walk.sort_by(|a, b| {
            let default = a.file_name().cmp(b.file_name());
            let asize = match a.metadata() {
                Ok(m) => m.len(),
                Err(_) => return default,
            };
            let bsize = match b.metadata() {
                Ok(m) => m.len(),
                Err(_) => return default,
            };
            asize.cmp(&bsize)
        }),
        Some(Sortby::Name) => walk.sort_by_file_name(),
        None => walk,
    }
}

pub fn get_dir_entries(visitor: &DirVisitor) -> (Vec<DirEntry>, usize) {
    let walk = get_walkdir(
        visitor.get_dir(),
        visitor.allow_symlinks(),
        visitor.get_sorter(),
        visitor.get_max_depth(),
    )
    .into_iter()
    .skip(1)
    .filter_map(|e| e.ok())
    .filter(|e| match (visitor.is_all(), visitor.is_fileonly()) {
        (true, false) => true,
        (true, true) => e.path().is_file(),
        (false, false) => !e.file_name().as_encoded_bytes().starts_with(b"."),
        (false, true) => !e.file_name().as_encoded_bytes().starts_with(b".") && e.path().is_file(),
    })
    .collect::<Vec<DirEntry>>();

    let walk_size = walk.len();
    (walk, walk_size)
}
