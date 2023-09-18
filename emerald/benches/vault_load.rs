use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use emerald::emerald::Emerald;

fn vault_load() {
    let vault_path = PathBuf::from("./tests/test_vault");
    let _emerald = Emerald::new(&vault_path).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vault_load", |b| b.iter(|| vault_load()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
