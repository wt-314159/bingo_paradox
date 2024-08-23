use std::{fmt, time::Instant};
use rand::{self, Rng, thread_rng};
use bingo_paradox::BingoCard;

const SIZE: usize = 10_000;

fn main() {
    run_bingo_with_many_cards()
}

fn run_bingo_with_many_cards() {
    let mut cards: Vec<_> = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        cards.push(BingoCard::new());
    }

    let mut rng = thread_rng();
    let mut rand = rng.gen_range(0..=75);
    let mut count = 1;
    println!("Number {}!", rand);

    while !cards.iter_mut().any(|x| x.test_match(rand)) {
        rand = rng.gen_range(0..=75);
        println!("Number {}!", rand);
        count += 1;
    }

    println!("Bingo! After {} balls were drawn", count);
    let winning_card = cards.iter()
        .find(|x| x.test_bingo())
        .expect("No winning card found!");
    println!("Winning bingo card: \n{}", winning_card);
}

fn test_bingo_works() {
    let mut test_card = BingoCard::new();

    for i in 0..=75 {
        if test_card.test_match(i) {
            break;
        }
    }
    println!("{}", test_card);
}

fn test_bingo_works_random() {
    let mut test_card = BingoCard::new();
    let mut rng = thread_rng();
    let mut rand = rng.gen_range(0..=75);
    let mut count = 1;

    while !test_card.test_match(rand) {
        rand = rng.gen_range(0..=75);
        count += 1;
    }
    println!("Bingo! After {} balls were drawn", count);
    println!("{}", test_card);
}

#[cfg(test)]
mod tests {
    use super::*;


}