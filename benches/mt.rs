use criterion::{criterion_group, criterion_main, Criterion};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod benchutil;

fn suite(c: &mut Criterion) {
    let entries = &benchutil::get_entries();
    c.bench_function("Multi-thread indexing", |b| {
        b.iter(|| {
            entries.par_iter().for_each(|entry| {
                if entry.file_type().unwrap().is_file() {
                    let _ = win_tcache_multi::bindings::__bench_force_get_thumbnail_from_path(
                        entry.path(),
                    );
                }
            });
        });
    });
}

criterion_group! {
    name = mt;
    config = Criterion::default().sample_size(10);
    targets = suite
}

criterion_main!(mt);
