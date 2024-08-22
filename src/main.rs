use std::{fmt, time::Instant};
use rand::{self, Rng, thread_rng};

const SIZE: usize = 1_000_000;

fn main() {
    let mut test = BingoCard::new();
    let mut rng = thread_rng();
    let mut randoms: [usize; SIZE] = [0; SIZE];

    let now = Instant::now();
    for i in 0..SIZE {
        randoms[i] = rng.gen_range(1..=75);
    }
    println!("Randoms generated in {:#?}", now.elapsed());

    let mut count = 0;
    let now = Instant::now();
    for i in 0..SIZE {
        if test.contains_number(randoms[i]) {
            count += 1;
        }
    }
    println!("There were {} matches in {:#?}", count, now.elapsed());

    count = 0;
    let now = Instant::now();
    for i in 0..SIZE {
        if test.find_match(randoms[i]) {
            count += 1;
        }
    }
    println!("There were {} matches in {:#?}", count, now.elapsed());
}

struct BingoCard {
    numbers: [[usize;5];5],
    matches: [[bool;5];5],
}

impl BingoCard {
    pub fn new() -> BingoCard {
        let mut rng = thread_rng();
        let mut numbers = [[0;5];5];

        for i in 0..5 {
            let min = i * 15 + 1;
            let max = (i + 1) * 15;
            let range = min..=max;
            println!("Generating numbers from {} to {}", min, max);
            for j in 0..5 {
                let range_clone = range.clone();
                numbers[j][i] = rng.gen_range(range_clone);
            }
        }

        numbers[2][2] = 0;
        BingoCard { numbers, matches: [[false;5];5] }
    }

    pub fn contains_number(&self, number: usize) -> bool {
        self.numbers.iter().any(|x| x.contains(&number))
    }

    pub fn find_match(&self, number: usize) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    return true;
                }
            }
        }
        return false;
    }
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                let result = match self.matches[i][j] {
                    true => write!(f, "-{:-<2}- ", self.numbers[i][j]),
                    false => write!(f, " {:<4}", self.numbers[i][j])
                };
                if result.is_err() {
                    return result;
                }
            }
            if let Err(err) = writeln!(f) {
                return Err(err);
            }
        }
        Ok(())
    }
}