use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use emerald::emerald::Emerald;

fn vault_load() {
    let vault_path = PathBuf::from("./tests/test_vault");
    let _emerald = Emerald::new(&vault_path).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-2000");
    group.sample_size(2000);
    group.bench_function("vault_load", |b| b.iter(|| vault_load()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
