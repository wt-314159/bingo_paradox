use std::fmt;
use rand::{
    self, 
    Rng, 
    thread_rng} ;
use core::ops::RangeInclusive;

const SOURCE_NUMBERS: [[usize;15];5] = [
    [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
    [31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
    [46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
    [61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75]];

const SOURCE_RANGES: [RangeInclusive<usize>;5] = [
    1..=15, 
    16..=30,
    31..=45, 
    45..=60, 
    61..=75];

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

    pub fn new_cached_ranges() -> BingoCard {
        let mut rng = thread_rng();
        let mut numbers = [[0;5];5];

        for i in 0..5 {
            for j in 0..5 {
                let range_clone = SOURCE_RANGES[i].clone();
                numbers[j][i] = rng.gen_range(range_clone);
            }
        }

        numbers[2][2] = 0;
        BingoCard { numbers, matches: [[false;5];5] }
    }

    pub fn new_alternate() -> BingoCard {
        let mut rng = thread_rng();
        let mut numbers = [[0;5];5];

        for i in 0..5 {
            for j in 0..5 {
                let random = rng.gen_range(0..15);
                numbers[j][i] = SOURCE_NUMBERS[i][random];
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

pub fn gen_range_no_duplicates_alternate(range: RangeInclusive<usize>) -> [usize; 5]
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

pub fn contains_number_contains(array: [usize;5], number: usize) -> bool {
    array.contains(&number)
}

pub fn contains_number_for(array: [usize;5], number: usize) -> bool {
    for i in 0..array.len() {
        if array[i] == number {
            return true;
        }
    }
    false
}

pub fn contains_number_foreach(array: [usize;5], number: usize) -> bool {
    for element in array {
        if element == number {
            return true;
        }
    }
    false
}

pub fn contains_number_iter_any(array: [usize;5], number: usize) -> bool {
    array.iter().any(|x| x == &number)
}