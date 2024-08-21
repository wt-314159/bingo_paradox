use std::fmt;
use rand::{self, Rng, thread_rng};

fn main() {
    let mut test = BingoCard::new();
    test.matches[0][0] = true;
    test.matches[2][2] = true;
    test.matches[1][4] = true;
    test.matches[0][1] = true;
    println!();
    println!("{test}");
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