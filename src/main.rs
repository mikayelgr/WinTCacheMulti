use std::fs::DirEntry;
use std::ptr::null_mut;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Instant;
use std::{io, path::PathBuf};

use clap::Parser;

use win_tcache_multi::{bindings, sys};

#[derive(clap::Parser)]
struct Args {
    /// The directory to index in the thumbnail cache.
    #[clap(long, short, name = "directory")]
    dir: PathBuf,
    #[clap(action, long, short)]
    st: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Safe method for closing the component object model
    ctrlc::set_handler(|| unsafe { sys::CoUninitialize() })
        .expect("Couldn't register Ctrl-C handler");
    let args: Args = Args::parse();

    // Normalize Windows paths to the most compatible format, avoiding UNC where possible
    // This helps us avoid issues where the std library converts the provided path into
    // a string prefixed with `\\?\`.
    let path = dunce::canonicalize(args.dir)?;
    let entries: Vec<DirEntry> = std::fs::read_dir(path)?.map(|e| e.unwrap()).collect();
    let indexed = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();

    if !args.st {
        // Initializing in MTA mode
        unsafe { assert_eq!(sys::CoInitializeEx(null_mut(), 0x0), 0) };
        let mut tasks = vec![];
        for e in entries {
            let indexed = indexed.clone();
            let t = tokio::task::spawn_blocking(move || {
                assert_eq!(unsafe { sys::CoInitializeEx(null_mut(), 0x0) }, 0);
                if e.file_type().unwrap().is_file() {
                    match bindings::__bench_force_get_thumbnail_from_path(e.path()) {
                        Err(e) => eprintln!("{:?}", e),
                        _ => {
                            println!("{:?}", e.path());
                            indexed.fetch_add(1, std::sync::atomic::Ordering::Release);
                        }
                    }
                }
            });

            tasks.push(t);
        }

        for t in tasks {
            t.await?;
        }
    } else {
        // Initializing in STA mode
        unsafe { assert_eq!(sys::CoInitialize(null_mut()), 0x0) };

        for e in entries {
            if e.file_type().unwrap().is_file() {
                match bindings::__bench_force_get_thumbnail_from_path(e.path()) {
                    Err(e) => eprintln!("{:?}", e),
                    _ => {
                        indexed.fetch_add(1, std::sync::atomic::Ordering::Release);
                        println!("{:?}", e.path());
                    }
                }
            }
        }
    }

    unsafe { sys::CoUninitialize() };
    println!(
        "Successfully indexed {} items in {}ms",
        indexed.load(std::sync::atomic::Ordering::Acquire),
        start.elapsed().as_millis(),
    );

    Ok(())
}
