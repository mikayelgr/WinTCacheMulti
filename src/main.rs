use std::fs::DirEntry;
use std::{io, path::PathBuf};

use clap::Parser;

use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use win_tcache_multi::bindings;

#[derive(clap::Parser)]
struct Args {
    /// The directory to index in the thumbnail cache.
    #[clap(name = "directory")]
    dir: PathBuf,
}

fn main() -> io::Result<()> {
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

    bindings::coinitialize_mta()?;
    entries.par_iter().for_each(|entry| {
        if entry.file_type().unwrap().is_file() {
            let _ = bindings::get_thumbnail_from_path(entry.path());
        }
    });

    bindings::couninitialize();
    Ok(())
}
