use chess_engine::bitboard;

use criterion::{criterion_group, criterion_main, Criterion};

fn flip_bitboard_horizontal_benchmark(c: &mut Criterion) {
    c.bench_function("flip bitboard horizontal", |b| {
        b.iter(|| bitboard::flip_bitboard_over_horizontal(4832908290849048_u64))
    });
}

criterion_group!(benches, flip_bitboard_horizontal_benchmark);
criterion_main!(benches);
