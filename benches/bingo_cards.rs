use criterion::{criterion_group, criterion_main, Criterion};
use bingo_paradox::{gen_range_no_duplicates, BingoCard};

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
        b.iter(|| rand_numbers.contains(&test_value))
    });

    group.bench_with_input("contains_number_for", &rand_numbers, |b,i| {
        b.iter(|| contains_number_for(*i, test_value))
    });

    group.bench_with_input("contains_number_foreach", &rand_numbers, |b,i| {
        b.iter(|| contains_number_foreach(*i, test_value))
    });

    group.bench_with_input("contains_number_any", &rand_numbers, |b,i| {
        b.iter(|| contains_number_iter_any(*i, test_value))
    });

    group.finish();
}

fn bench_test_match(c: &mut Criterion) {
    let mut card = BingoCard::new();
    for i in (0..=3).step_by(3) {
        card.test_match(i);
    }

    c.bench_function("bench_test_match", |b| {
        b.iter(|| card.test_match(42))
    });
}

criterion_group!(
    benches,
    // bench_contains_number,
    bench_test_match,
);

criterion_main!(benches);


fn contains_number_for(array: [usize;5], number: usize) -> bool {
    for i in 0..array.len() {
        if array[i] == number {
            return true;
        }
    }
    false
}

fn contains_number_foreach(array: [usize;5], number: usize) -> bool {
    for element in array {
        if element == number {
            return true;
        }
    }
    false
}

fn contains_number_iter_any(array: [usize;5], number: usize) -> bool {
    array.iter().any(|x| x == &number)
}