use criterion::{criterion_group, criterion_main, Criterion};

mod benchutil;

fn suite(c: &mut Criterion) {
    let entries: &Vec<std::fs::DirEntry> = &benchutil::get_entries();
    c.bench_function("Single-thread indexing", |b| {
        b.iter(|| {
            for entry in entries {
                if entry.file_type().unwrap().is_file() {
                    let _ = win_tcache_multi::bindings::__bench_force_get_thumbnail_from_path(
                        entry.path(),
                    );
                }
            }
        });
    });
}

criterion_group! {
    name = mt;
    config = Criterion::default().sample_size(10);
    targets = suite
}

criterion_main!(mt);
