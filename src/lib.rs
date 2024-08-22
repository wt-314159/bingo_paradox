use std::fmt;
use rand::{
    self, 
    Rng, 
    thread_rng} ;
use core::ops::RangeInclusive;

pub struct BingoCard {
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
            
            for j in 0..5 {
                let range_clone = range.clone();
                numbers[j][i] = rng.gen_range(range_clone);

            }
        }

        numbers[2][2] = 0;
        BingoCard { numbers, matches: [[false;5];5] }
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

pub fn gen_range_no_duplicates(range: RangeInclusive<usize>) -> [usize; 5]
{
    let mut values = [0; 5];
    let mut rng = thread_rng();

    for i in 0..5 {
        let mut rand = rng.gen_range(range.clone());
        while values[0..i].contains(&rand) {
            rand = rng.gen_range(range.clone());
        }
        values[i] = rand;
    }

    values
}