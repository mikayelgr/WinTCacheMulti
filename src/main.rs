use std::fs::DirEntry;
use std::ptr::null_mut;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use std::{io, path::PathBuf};

use clap::Parser;

use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use win_tcache_multi::bindings::{self, extra};

#[derive(clap::Parser)]
struct Args {
    /// The directory to index in the thumbnail cache.
    #[clap(long, short, name = "directory")]
    dir: PathBuf,
    #[clap(action, long, short)]
    st: bool,
}

fn main() -> io::Result<()> {
    // https://learn.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities
    // unsafe { bindings::extra::SetPriorityClass(bindings::extra::GetCurrentProcess(), 31) };

    let args: Args = Args::parse();
    // Normalize Windows paths to the most compatible format, avoiding UNC where possible
    // This helps us avoid issues where the std library converts the provided path into
    // a string prefixed with `\\?\`.
    let path = dunce::canonicalize(args.dir)?;

    // Unwrapping the entries in parallel
    let entries: Vec<DirEntry> = std::fs::read_dir(path)?
        .par_bridge() // Converts the iterator into a parallel iterator
        .map(|e| e.unwrap())
        .collect();

    let indexed = AtomicUsize::new(0);
    let indexed_ref = &indexed;
    let start = Instant::now();

    if !args.st {
        // TODO: Initialize the COM in MTA mode
        unsafe { assert_eq!(extra::CoInitializeEx(null_mut(), 0x0), 0) };
        entries.par_iter().for_each(|entry| {
            if entry.file_type().unwrap().is_file() {
                let fetch = bindings::get_thumbnail_from_path(entry.path());
                if fetch.is_ok() {
                    indexed_ref.fetch_add(1, std::sync::atomic::Ordering::Release);
                }
            }
        });
    } else {
        unsafe { assert_eq!(extra::CoInitialize(null_mut()), 0x0) };
        for entry in entries {
            if entry.file_type().unwrap().is_file() {
                let fetch = bindings::get_thumbnail_from_path(entry.path());
                if fetch.is_ok() {
                    indexed_ref.fetch_add(1, std::sync::atomic::Ordering::Release);
                }
            }
        }
    }

    unsafe { extra::CoUninitialize() };
    println!(
        "Successfully indexed {} items in {}ms",
        indexed.load(std::sync::atomic::Ordering::Acquire),
        start.elapsed().as_millis(),
    );

    Ok(())
}
