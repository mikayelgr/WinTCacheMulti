use std::ptr::null_mut;

use criterion::{criterion_group, criterion_main, Criterion};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use win_tcache_multi::bindings::extra;

mod benchutil;

fn mt(c: &mut Criterion) {
    c.bench_function("multi-threaded", |b| {
        unsafe { extra::CoInitializeEx(core::ptr::null_mut(), 0x0) };
        let entries = &benchutil::get_entries();
        b.iter(|| {
            entries.par_iter().for_each(|e| {
                let _ = win_tcache_multi::bindings::__bench_force_get_thumbnail_from_path(e.path());
            });
        });

        unsafe { extra::CoUninitialize() };
    });
}

fn st(c: &mut Criterion) {
    c.bench_function("single-threaded", |b| {
        unsafe { assert_eq!(extra::CoInitialize(null_mut()), 0x0) };
        let entries = &benchutil::get_entries();
        b.iter(|| {
            for e in entries {
                let _ = win_tcache_multi::bindings::__bench_force_get_thumbnail_from_path(e.path());
            }
        });

        unsafe { extra::CoUninitialize() };
    });
}

criterion_group! {
    name = xthread;
    config = Criterion::default().sample_size(10);
    targets = mt, st,
}

criterion_main!(xthread);
