use criterion::{criterion_group, criterion_main, Criterion};
use bingo_paradox::{gen_range_no_duplicates, BingoCard};

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

fn bench_gen_range_no_duplicates(c: &mut Criterion) {
    c.bench_function("bench_gen_range_no_duplicates", |b| {
        b.iter(|| bingo_paradox::gen_range_no_duplicates(1..=15))
    });
}

fn bench_contains_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_number");
    let rand_numbers = gen_range_no_duplicates(1..=100);
    let test_value = 42;

    group.bench_with_input("contains_number", &rand_numbers, |b,i| {
        b.iter(|| bingo_paradox::contains_number_contains(*i, test_value))
    });

    group.bench_with_input("contains_number_for", &rand_numbers, |b,i| {
        b.iter(|| bingo_paradox::contains_number_for(*i, test_value))
    });

    group.bench_with_input("contains_number_foreach", &rand_numbers, |b,i| {
        b.iter(|| bingo_paradox::contains_number_foreach(*i, test_value))
    });

    group.bench_with_input("contains_number_any", &rand_numbers, |b,i| {
        b.iter(|| bingo_paradox::contains_number_iter_any(*i, test_value))
    });

    group.finish();
}

criterion_group!(
    benches,
    // bench_new,
    // bench_new_alternate,
    // bench_new_cached_ranges,
    //bench_gen_range_no_duplicates,
    bench_contains_number
);

criterion_main!(benches);