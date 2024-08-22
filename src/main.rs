use std::{fmt, time::Instant};
use rand::{self, Rng, thread_rng};
use bingo_paradox::BingoCard;

const SIZE: usize = 10_000;

fn main() {
    let mut cards: Vec<_> = Vec::with_capacity(SIZE);

    let now = Instant::now();
    for _ in 0..SIZE {
        let card = BingoCard::new();
        cards.push(card);
    }
    println!("{} cards generated in {:#?}", SIZE, now.elapsed());

    let mut rng = thread_rng();
    let rand = rng.gen_range(1..=75);

    let mut count = 0;
    let now = Instant::now();
    for card in cards {
        if card.find_match(rand) {
            count += 1;
        }
    }
    println!("Number {} was found on {} cards in {:#?}", rand, count, now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
}