use chess_engine::bitboard::Bitboard;

use criterion::{criterion_group, criterion_main, Criterion};

fn flip_bitboard_horizontal_benchmark(c: &mut Criterion) {
    c.bench_function("flip bitboard horizontal", |b| {
        b.iter(|| 4832908290849048_u64.flip_over_horizontal())
    });
}

criterion_group!(benches, flip_bitboard_horizontal_benchmark);
criterion_main!(benches);
