use std::fs::DirEntry;

use rayon::iter::{ParallelBridge, ParallelIterator};

#[inline]
pub fn get_entries() -> Vec<DirEntry> {
    let path = dunce::canonicalize("./benches/imgdata").unwrap();
    // Unwrapping the entries in parallel
    let mut entries: Vec<DirEntry> = std::fs::read_dir(&path)
        .unwrap()
        .par_bridge() // Converts the iterator into a parallel iterator
        .map(|e| e.unwrap())
        .collect();

    if let Some(gitkeep_index) = entries.iter().position(|e| e.file_name() == ".gitkeep") {
        entries.remove(gitkeep_index);
    }

    entries
}
