use std::ptr::null_mut;

use criterion::{criterion_group, criterion_main, Criterion};

use win_tcache_multi::sys;

mod benchutil;

// Multi-threaded benchmark
fn mt(c: &mut Criterion) {
    c.bench_function("multi-threaded", |b| {
        unsafe { assert_eq!(sys::CoInitializeEx(core::ptr::null_mut(), 0x0), 0) };
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let entries = benchutil::get_entries();
                let mut tasks = vec![];
                for e in entries {
                    let t = tokio::task::spawn_blocking(move || {
                        if e.file_type().unwrap().is_file() {
                            let r =
                                win_tcache_multi::bindings::__bench_force_get_thumbnail_from_path(
                                    e.path(),
                                );
                            assert!(r.is_ok());
                        }
                    });

                    tasks.push(t);
                }

                // Waiting for all the tasks to complete
                for t in tasks {
                    t.await.unwrap();
                }
            });
        });

        unsafe { sys::CoUninitialize() };
    });
}

// Single-threaded benchmark
fn st(c: &mut Criterion) {
    c.bench_function("single-threaded", |b| {
        unsafe { assert_eq!(sys::CoInitialize(null_mut()), 0) };
        let entries = &benchutil::get_entries();
        b.iter(|| {
            for e in entries {
                let r = win_tcache_multi::bindings::__bench_force_get_thumbnail_from_path(e.path());
                assert!(r.is_ok());
            }
        });

        unsafe { sys::CoUninitialize() };
    });
}

criterion_group! {
    name = xthread;
    config = Criterion::default().sample_size(10);
    targets = mt, st,
}

criterion_main!(xthread);
