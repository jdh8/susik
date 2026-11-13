//! Cards and hands.
//!
//! [`Card`] pairs a [`Color`] with a [`Rank`].  The deck holds four copies of
//! each of the 28 cards, so a [`Hand`] is a multiset: 28 four-bit counts
//! packed into a `u128` with a documented layout — the count of the card at
//! [`Card::index`] `i` is the nibble at bits `4i..4i+4`, and
//! [`Hand::FULL`] is the whole 112-card deck.
//!
//! Iteration yields cards with multiplicity in index order: all red cards
//! general first, then yellow, green, and white.
//!
//! # Panic policy
//!
//! [`Hand::with`] and [`Hand::without`] panic when the count would leave
//! `0..=4`; [`Hand::insert`] and [`Hand::remove`] report the same cases with
//! a `bool` instead.

use crate::{Color, InvalidColor, InvalidRank, Rank};
use core::fmt::{self, Write as _};
use core::iter::FusedIterator;
use core::ops;
use core::str::FromStr;
use thiserror::Error;

/// A card: a piece printed on a colour
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    /// The colour of the card stock
    pub color: Color,
    /// The piece printed on it
    pub rank: Rank,
}

impl Card {
    /// The 28 distinct cards in index order
    pub const ALL: [Self; 28] = {
        let mut cards = [Self::new(Color::Red, Rank::General); 28];
        let mut i = 0;
        while i < 28 {
            cards[i] = Self::from_index(i);
            i += 1;
        }
        cards
    };

    /// Create a card
    #[must_use]
    #[inline]
    pub const fn new(color: Color, rank: Rank) -> Self {
        Self { color, rank }
    }

    /// Position in `0..28`: `7 × color + rank`
    #[must_use]
    #[inline]
    pub const fn index(self) -> usize {
        self.color as usize * 7 + self.rank as usize
    }

    /// The card at an [`index`](Self::index)
    ///
    /// # Panics
    ///
    /// When `index >= 28`.
    #[must_use]
    #[inline]
    pub const fn from_index(index: usize) -> Self {
        Self::new(Color::ALL[index / 7], Rank::ALL[index % 7])
    }

    /// The character printed on this card
    #[must_use]
    #[inline]
    pub const fn glyph(self) -> char {
        self.rank.glyph(self.color)
    }
}

/// Error parsing a [`Card`]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseCardError {
    /// Not exactly two characters
    #[error("a card is a colour followed by a piece, e.g. 紅帥 or RK")]
    Length,
    /// Bad colour
    #[error(transparent)]
    Color(#[from] InvalidColor),
    /// Bad piece
    #[error(transparent)]
    Rank(#[from] InvalidRank),
}

impl fmt::Display for Card {
    /// The Chinese name of the colour followed by the printed character,
    /// e.g. `紅帥`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.color.name())?;
        f.write_char(self.glyph())
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    /// Accepts the display form or the two ASCII letters, e.g. `紅帥` or
    /// `RK`, in either case; the piece may be written with either set
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(color), Some(rank), None) = (chars.next(), chars.next(), chars.next()) else {
            return Err(ParseCardError::Length);
        };
        Ok(Self::new(Color::try_from(color)?, Rank::try_from(rank)?))
    }
}

/// A multiset of cards, up to four of each
///
/// The count of the card at index `i` lives in bits `4i..4i+4` of the
/// `u128`; the top 16 bits are always zero.  This layout is a stable API.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Hand(u128);

impl Hand {
    /// No cards
    pub const EMPTY: Self = Self(0);

    /// The whole deck: four of every card
    pub const FULL: Self = Self(0x4444_4444_4444_4444_4444_4444_4444);

    const fn shift(card: Card) -> u32 {
        4 * card.index() as u32
    }

    /// The raw counts
    #[must_use]
    #[inline]
    pub const fn to_bits(self) -> u128 {
        self.0
    }

    /// A hand from raw counts, if every nibble is at most 4 and the top 16
    /// bits are clear
    #[must_use]
    pub const fn from_bits(bits: u128) -> Option<Self> {
        let mut rest = bits;
        let mut i = 0;
        while i < 28 {
            if rest & 0xF > 4 {
                return None;
            }
            rest >>= 4;
            i += 1;
        }
        if rest == 0 { Some(Self(bits)) } else { None }
    }

    /// A hand from raw counts, trusting the caller
    #[must_use]
    #[inline]
    pub const fn from_bits_retain(bits: u128) -> Self {
        Self(bits)
    }

    /// How many copies of `card`
    #[must_use]
    #[inline]
    pub const fn count(self, card: Card) -> u8 {
        ((self.0 >> Self::shift(card)) & 0xF) as u8
    }

    /// Whether at least one copy of `card` is present
    #[must_use]
    #[inline]
    pub const fn contains(self, card: Card) -> bool {
        self.count(card) > 0
    }

    /// Add one copy of `card`; `false` (unchanged) if four are already held
    #[inline]
    pub const fn insert(&mut self, card: Card) -> bool {
        if self.count(card) >= 4 {
            return false;
        }
        self.0 += 1 << Self::shift(card);
        true
    }

    /// Take one copy of `card`; `false` (unchanged) if none is held
    #[inline]
    pub const fn remove(&mut self, card: Card) -> bool {
        if self.count(card) == 0 {
            return false;
        }
        self.0 -= 1 << Self::shift(card);
        true
    }

    /// This hand plus one copy of `card`
    ///
    /// # Panics
    ///
    /// When four copies are already held.
    #[must_use]
    #[inline]
    pub const fn with(mut self, card: Card) -> Self {
        assert!(self.insert(card), "already holding four copies");
        self
    }

    /// This hand minus one copy of `card`
    ///
    /// # Panics
    ///
    /// When no copy is held.
    #[must_use]
    #[inline]
    pub const fn without(mut self, card: Card) -> Self {
        assert!(self.remove(card), "no copy to remove");
        self
    }

    /// Total number of cards, with multiplicity
    #[must_use]
    pub const fn len(self) -> usize {
        let mut bits = self.0;
        let mut n = 0;
        while bits != 0 {
            n += (bits & 0xF) as usize;
            bits >>= 4;
        }
        n
    }

    /// Whether no card is held
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every card here is held at least as many times in `other`
    #[must_use]
    pub const fn is_subset(self, other: Self) -> bool {
        let (mut a, mut b) = (self.0, other.0);
        while a != 0 {
            if a & 0xF > b & 0xF {
                return false;
            }
            a >>= 4;
            b >>= 4;
        }
        true
    }

    /// The lowest-indexed card held
    #[must_use]
    #[inline]
    pub const fn first(self) -> Option<Card> {
        if self.is_empty() {
            None
        } else {
            Some(Card::from_index(self.0.trailing_zeros() as usize / 4))
        }
    }

    /// Cards with multiplicity, in index order
    #[inline]
    pub const fn iter(self) -> Iter {
        Iter(self.0)
    }

    /// Distinct cards with their counts, in index order
    pub fn distinct(self) -> impl Iterator<Item = (Card, u8)> {
        Card::ALL
            .into_iter()
            .map(move |card| (card, self.count(card)))
            .filter(|&(_, n)| n > 0)
    }
}

/// Iterator over the cards of a [`Hand`] with multiplicity
#[derive(Debug, Clone)]
pub struct Iter(u128);

impl Iterator for Iter {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        if self.0 == 0 {
            return None;
        }
        let index = self.0.trailing_zeros() as usize / 4;
        self.0 -= 1 << (4 * index);
        Some(Card::from_index(index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = Hand(self.0).len();
        (n, Some(n))
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Card> {
        if self.0 == 0 {
            return None;
        }
        let index = (127 - self.0.leading_zeros() as usize) / 4;
        self.0 -= 1 << (4 * index);
        Some(Card::from_index(index))
    }
}

impl ExactSizeIterator for Iter {}
impl FusedIterator for Iter {}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = Iter;

    fn into_iter(self) -> Iter {
        self.iter()
    }
}

impl FromIterator<Card> for Hand {
    /// Copies beyond the fourth are dropped
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let mut hand = Self::EMPTY;
        hand.extend(iter);
        hand
    }
}

impl Extend<Card> for Hand {
    /// Copies beyond the fourth are dropped
    fn extend<I: IntoIterator<Item = Card>>(&mut self, iter: I) {
        for card in iter {
            self.insert(card);
        }
    }
}

impl ops::Add for Hand {
    type Output = Self;

    /// Multiset sum
    ///
    /// # Panics
    ///
    /// When any card would exceed four copies.
    fn add(self, rhs: Self) -> Self {
        let sum = Self(self.0 + rhs.0);
        assert!(
            Self::from_bits(sum.0).is_some(),
            "more than four copies of a card"
        );
        sum
    }
}

impl ops::AddAssign for Hand {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Hand {
    type Output = Self;

    /// Multiset difference
    ///
    /// # Panics
    ///
    /// When `rhs` is not a subset of `self`.
    fn sub(self, rhs: Self) -> Self {
        assert!(rhs.is_subset(self), "subtracting cards not held");
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for Hand {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Error parsing a [`Hand`]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseHandError {
    /// Not four dot-separated groups
    #[error("a hand is four dot-separated colour groups: red.yellow.green.white")]
    Groups,
    /// Bad piece
    #[error(transparent)]
    Rank(#[from] InvalidRank),
    /// More than four of a card
    #[error("more than four copies of {0}")]
    TooMany(Card),
}

impl fmt::Display for Hand {
    /// Four dot-separated colour groups — red, yellow, green, white — each
    /// listing its pieces with multiplicity, e.g. `帥帥仕.俥.將.卒卒`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, color) in Color::ALL.into_iter().enumerate() {
            if i > 0 {
                f.write_char('.')?;
            }
            for rank in Rank::ALL {
                let card = Card::new(color, rank);
                for _ in 0..self.count(card) {
                    f.write_char(card.glyph())?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    /// Parses the display form; pieces may be written with either set or
    /// their ASCII letters
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = s.split('.');
        let mut hand = Self::EMPTY;
        for color in Color::ALL {
            let group = groups.next().ok_or(ParseHandError::Groups)?;
            for c in group.chars() {
                let card = Card::new(color, Rank::try_from(c)?);
                if !hand.insert(card) {
                    return Err(ParseHandError::TooMany(card));
                }
            }
        }
        if groups.next().is_some() {
            return Err(ParseHandError::Groups);
        }
        Ok(hand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_round_trip() {
        for card in Card::ALL {
            assert_eq!(Card::from_index(card.index()), card);
            assert_eq!(card.to_string().parse(), Ok(card));
            let code: String = [card.color.letter(), card.rank.letter()]
                .into_iter()
                .collect();
            assert_eq!(code.parse(), Ok(card));
            assert_eq!(code.to_lowercase().parse(), Ok(card));
        }
        assert_eq!(
            "紅帥".parse::<Card>(),
            Ok(Card::new(Color::Red, Rank::General))
        );
        assert_eq!(
            "紅將".parse::<Card>(),
            Ok(Card::new(Color::Red, Rank::General))
        );
        assert_eq!("WP".parse::<Card>().unwrap().to_string(), "白卒");
        assert_eq!("紅".parse::<Card>(), Err(ParseCardError::Length));
        assert_eq!("紅帥仕".parse::<Card>(), Err(ParseCardError::Length));
        assert_eq!("X帥".parse::<Card>(), Err(InvalidColor('X').into()));
        assert_eq!("紅X".parse::<Card>(), Err(InvalidRank('X').into()));
    }

    #[test]
    fn hands_are_multisets() {
        let hand: Hand = "帥帥仕.俥.將.卒卒".parse().unwrap();
        assert_eq!(hand.len(), 7);
        assert_eq!(hand.to_string(), "帥帥仕.俥.將.卒卒");
        assert_eq!("KKA.R.K.PP".parse::<Hand>(), Ok(hand));
        let red_general = Card::new(Color::Red, Rank::General);
        assert_eq!(hand.count(red_general), 2);
        assert_eq!(hand.first(), Some(red_general));
        assert_eq!(hand.iter().collect::<Hand>(), hand);
        assert_eq!(hand.iter().count(), 7);
        assert_eq!(hand.iter().len(), 7);
        assert_eq!(
            hand.iter().next_back(),
            Some(Card::new(Color::White, Rank::Soldier))
        );
        assert_eq!(hand.distinct().count(), 5);
        assert!(hand.is_subset(Hand::FULL));
        assert!(!Hand::FULL.is_subset(hand));
        assert_eq!(
            hand - hand.without(red_general),
            [red_general].into_iter().collect()
        );
        assert_eq!(Hand::FULL.len(), 112);
        assert_eq!(Hand::FULL.iter().count(), 112);
        assert_eq!(Hand::from_bits(Hand::FULL.to_bits()), Some(Hand::FULL));
        assert_eq!(Hand::from_bits(0x5), None);
        assert_eq!(Hand::from_bits(1 << 112), None);
        assert_eq!(Hand::EMPTY.to_string(), "...");
        assert_eq!("...".parse::<Hand>(), Ok(Hand::EMPTY));
        assert_eq!("..".parse::<Hand>(), Err(ParseHandError::Groups));
        assert_eq!("....".parse::<Hand>(), Err(ParseHandError::Groups));
        assert_eq!(
            "帥帥帥帥帥...".parse::<Hand>(),
            Err(ParseHandError::TooMany(red_general))
        );
        assert_eq!("X...".parse::<Hand>(), Err(InvalidRank('X').into()));
    }

    #[test]
    #[should_panic(expected = "four copies")]
    fn five_copies_panic() {
        let _ = "帥帥帥帥..."
            .parse::<Hand>()
            .unwrap()
            .with(Card::new(Color::Red, Rank::General));
    }
}
