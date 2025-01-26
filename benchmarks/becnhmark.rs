[dev-dependencies]
criterion = "0.4"

use criterion::{black_box, Criterion};

fn bench_neuro_lz77(c: &mut Criterion) {
    let data = include_bytes!("../benchmarks/calgary/book1");
    c.bench_function("neuro_lz77", |b| b.iter(|| {
        NeuroLz77::new().compress(black_box(data))
    }));
}

Metrics:

Compression ratio (original/compressed).

Throughput (MB/s) for encode/decode.

Memory usage (e.g., heapsize crate).