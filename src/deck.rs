//! Shuffling the deck.
//!
//! This module is gated behind the `rand` feature.

use crate::{Card, Hand};
use rand::{Rng, RngExt as _};

/// The whole deck in a uniformly random order
///
/// Games draw from the end of the returned vector so that `pop` is the deal.
#[must_use]
pub fn shuffle(rng: &mut (impl Rng + ?Sized)) -> Vec<Card> {
    let mut cards: Vec<Card> = Hand::FULL.iter().collect();
    for i in (1..cards.len()).rev() {
        cards.swap(i, rng.random_range(..=i));
    }
    cards
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng as _;
    use rand::rngs::SmallRng;

    #[test]
    fn shuffling_permutes_the_deck() {
        let mut rng = SmallRng::seed_from_u64(0);
        let deck = shuffle(&mut rng);
        assert_eq!(deck.len(), 112);
        assert_eq!(deck.iter().copied().collect::<Hand>(), Hand::FULL);
        assert_ne!(deck, shuffle(&mut rng));
    }
}
