use criterion::{criterion_group, criterion_main, Criterion};
use bingo_paradox::BingoCard;

fn bench_new(c: &mut Criterion) {
    c.bench_function("bench_new_bingo_card", |b| {
        b.iter(|| BingoCard::new())
    });
}

fn bench_new_alternate(c: &mut Criterion) {
    c.bench_function("bench_new_bingo_card_alternate", |b| {
        b.iter(|| BingoCard::new_alternate())
    });
}

fn bench_new_cached_ranges(c: &mut Criterion) {
    c.bench_function("bench_new_bingo_card_cached_ranges", |b| {
        b.iter(|| BingoCard::new_cached_ranges())
    });
}

criterion_group!(
    benches,
    bench_new,
    bench_new_alternate,
    bench_new_cached_ranges,
);

criterion_main!(benches);