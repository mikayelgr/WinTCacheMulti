use std::fs::DirEntry;

use rayon::iter::{ParallelBridge, ParallelIterator};
use win_tcache_multi::bindings;
// use win_tcache_multi::bindings::extra::{GetCurrentProcess, SetPriorityClass};

#[inline]
pub fn get_entries() -> Vec<DirEntry> {
    // https://learn.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities
    unsafe { bindings::extra::SetPriorityClass(bindings::extra::GetCurrentProcess(), 31) };

    let path = dunce::canonicalize("benches/imgdata").unwrap();
    // Unwrapping the entries in parallel
    std::fs::read_dir(&path)
        .unwrap()
        .par_bridge() // Converts the iterator into a parallel iterator
        .map(|e| e.unwrap())
        .collect()
}
