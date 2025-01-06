#![allow(non_snake_case)]

use std::fs::DirEntry;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use win_tcache_multi::bindings::{self, extra::GetCurrentProcess};

#[inline]
fn get_entries() -> Vec<DirEntry> {
    unsafe {
        bindings::extra::SetPriorityClass(GetCurrentProcess(), 31);
    };

    let path = dunce::canonicalize("benches/imgdata").unwrap();
    // Unwrapping the entries in parallel
    std::fs::read_dir(&path)
        .unwrap()
        .par_bridge() // Converts the iterator into a parallel iterator
        .map(|e| {
            let entry = e.unwrap();

            entry
        })
        .collect()
}

fn mta_multi__root_ctx(c: &mut Criterion) {
    let entries = get_entries();

    win_tcache_multi::bindings::coinitialize_mta().unwrap();
    c.bench_with_input(
        BenchmarkId::new(
            "Multi-threaded extraction in a multi-thread apartment (MTA) in ROOT context",
            "benches/imgdata",
        ),
        &entries,
        |b, entries| {
            b.iter(|| {
                black_box({
                    entries.par_iter().for_each(move |entry| {
                        if entry.file_type().unwrap().is_file() {
                            #[allow(unused)]
                            bindings::__bench_force_get_thumbnail_from_path(entry.path());
                        }
                    })
                });
            })
        },
    );

    win_tcache_multi::bindings::couninitialize();
}

fn sta_multi__root_ctx(c: &mut Criterion) {
    win_tcache_multi::bindings::coinitialize_sta().unwrap();
    let entries = get_entries();

    c.bench_with_input(
        BenchmarkId::new(
            "Multi-threaded extraction in a single-thread apartment (STA) in ROOT context",
            "benches/imgdata",
        ),
        &entries,
        |b, entries| {
            b.iter(|| {
                black_box({
                    entries.par_iter().for_each(move |entry| {
                        if entry.file_type().unwrap().is_file() {
                            #[allow(unused)]
                            bindings::__bench_force_get_thumbnail_from_path(entry.path());
                        }
                    })
                });
            })
        },
    );

    win_tcache_multi::bindings::couninitialize();
}

criterion_group! {
    name = xta_multi;
    config = Criterion::default().sample_size(10);
    targets =
        mta_multi__root_ctx,
        sta_multi__root_ctx,
}

criterion_main!(xta_multi);
