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

    pub fn create_with_numbers(numbers: [[usize;5];5]) -> BingoCard {
        BingoCard { numbers, matches: [[false;5];5] }
    }

    pub fn test_match(&mut self, number: usize) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    self.matches[i][j] = true;  
                    return self.test_bingo();
                }
            }
        }
        return false;
    }

    pub fn test_bingo(&self) -> bool {
        // test rows
        for i in 0..5 {
            if self.matches[i].iter().all(|x| *x == true) {
                return true;
            }
        }
        // test columns
        for j in 0..5 {
            let mut val = true;
            for i in 0..5 {
                if self.matches[i][j] == false {
                    val = false;
                    break;
                }
            }
            if val {
                return true;
            }
        }
        false
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