//! Melds and the layout solver.
//!
//! The games of this deck all build hands from the same shapes, listed in
//! [`Meld`]: a lone general, two to four identical cards, the three court
//! pieces or the three field pieces of one colour, and three or four
//! soldiers of distinct colours.  What each shape is worth — and whether a
//! lone general or a pair counts at all — is a game's business, expressed as
//! a [`Values`] table.
//!
//! [`completions`] lists the melds a card can complete with a hand, the
//! basis of claiming a public card.  [`Solver`] lays a hand out as melds
//! maximizing the total value, either exactly ([`partition`]) or allowing
//! loose cards at a cost, which bots use to evaluate a hand.

use crate::{Card, Color, Hand, Rank};
use core::fmt;
use std::collections::HashMap;

/// A set of colours, for the soldiers of a [`Meld::Soldiers`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ColorSet(u8);

impl ColorSet {
    /// No colours
    pub const EMPTY: Self = Self(0);

    /// All four colours
    pub const ALL: Self = Self(0xF);

    /// Whether `color` is in the set
    #[must_use]
    #[inline]
    pub const fn contains(self, color: Color) -> bool {
        self.0 & (1 << color as u8) != 0
    }

    /// Add a colour; `false` if it was already present
    #[inline]
    pub const fn insert(&mut self, color: Color) -> bool {
        let was = self.contains(color);
        self.0 |= 1 << color as u8;
        !was
    }

    /// Number of colours
    #[must_use]
    #[inline]
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    /// Whether the set is empty
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Colours in [`Color::ALL`] order
    pub fn iter(self) -> impl Iterator<Item = Color> {
        Color::ALL.into_iter().filter(move |&c| self.contains(c))
    }
}

impl FromIterator<Color> for ColorSet {
    fn from_iter<I: IntoIterator<Item = Color>>(iter: I) -> Self {
        let mut set = Self::EMPTY;
        for color in iter {
            set.insert(color);
        }
        set
    }
}

/// A meld, one of the shapes a hand is laid out in
///
/// `Soldiers` must hold three or four colours; the functions of this module
/// only ever produce such sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Meld {
    /// A lone general (將 / 帥)
    Single(Card),
    /// Two identical cards
    Pair(Card),
    /// Three identical cards (刻)
    Triplet(Card),
    /// Four identical cards (槓)
    Quad(Card),
    /// General, advisor, and elephant of one colour (將士象)
    Court(Color),
    /// Chariot, horse, and cannon of one colour (車馬包)
    Field(Color),
    /// Soldiers of three or four distinct colours (三色卒 / 四色卒)
    Soldiers(ColorSet),
}

impl Meld {
    /// The cards of the meld
    #[must_use]
    pub fn cards(&self) -> Hand {
        match *self {
            Self::Single(card) => [card].into_iter().collect(),
            Self::Pair(card) => [card; 2].into_iter().collect(),
            Self::Triplet(card) => [card; 3].into_iter().collect(),
            Self::Quad(card) => [card; 4].into_iter().collect(),
            Self::Court(color) => [Rank::General, Rank::Advisor, Rank::Elephant]
                .into_iter()
                .map(|rank| Card::new(color, rank))
                .collect(),
            Self::Field(color) => [Rank::Chariot, Rank::Horse, Rank::Cannon]
                .into_iter()
                .map(|rank| Card::new(color, rank))
                .collect(),
            Self::Soldiers(colors) => colors.iter().map(|c| Card::new(c, Rank::Soldier)).collect(),
        }
    }

    /// Number of cards in the meld
    #[must_use]
    pub const fn size(&self) -> usize {
        match *self {
            Self::Single(_) => 1,
            Self::Pair(_) => 2,
            Self::Triplet(_) | Self::Court(_) | Self::Field(_) => 3,
            Self::Quad(_) => 4,
            Self::Soldiers(colors) => colors.len(),
        }
    }

    /// Whether the meld contains `card`
    #[must_use]
    pub fn contains(&self, card: Card) -> bool {
        self.cards().contains(card)
    }
}

impl fmt::Display for Meld {
    /// The cards in [`Hand`] notation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cards().fmt(f)
    }
}

/// The value of each meld shape, a game's scoring table
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Values {
    /// A lone general
    pub single: u32,
    /// Two identical cards
    pub pair: u32,
    /// Three identical cards
    pub triplet: u32,
    /// Four identical cards
    pub quad: u32,
    /// 將士象
    pub court: u32,
    /// 車馬包
    pub field: u32,
    /// Soldiers of three colours
    pub three_soldiers: u32,
    /// Soldiers of four colours
    pub four_soldiers: u32,
}

impl Values {
    /// The value of a meld
    #[must_use]
    pub const fn score(&self, meld: &Meld) -> u32 {
        match *meld {
            Meld::Single(_) => self.single,
            Meld::Pair(_) => self.pair,
            Meld::Triplet(_) => self.triplet,
            Meld::Quad(_) => self.quad,
            Meld::Court(_) => self.court,
            Meld::Field(_) => self.field,
            Meld::Soldiers(colors) => {
                if colors.len() == 4 {
                    self.four_soldiers
                } else {
                    self.three_soldiers
                }
            }
        }
    }
}

/// The melds `card` completes with cards from `hand`
///
/// `hand` is the cards besides `card`.  Every returned meld contains `card`
/// and its other cards are held in `hand`.  All shapes are listed; a game
/// decides which may be claimed.
#[must_use]
pub fn completions(hand: Hand, card: Card) -> Vec<Meld> {
    let n = hand.count(card);
    let mut melds = Vec::new();
    if card.rank == Rank::General {
        melds.push(Meld::Single(card));
    }
    if n >= 1 {
        melds.push(Meld::Pair(card));
    }
    if n >= 2 {
        melds.push(Meld::Triplet(card));
    }
    if n >= 3 {
        melds.push(Meld::Quad(card));
    }
    let has = |rank| rank == card.rank || hand.contains(Card::new(card.color, rank));
    match card.rank {
        Rank::General | Rank::Advisor | Rank::Elephant => {
            if has(Rank::General) && has(Rank::Advisor) && has(Rank::Elephant) {
                melds.push(Meld::Court(card.color));
            }
        }
        Rank::Chariot | Rank::Horse | Rank::Cannon => {
            if has(Rank::Chariot) && has(Rank::Horse) && has(Rank::Cannon) {
                melds.push(Meld::Field(card.color));
            }
        }
        Rank::Soldier => {
            let others: Vec<Color> = Color::ALL
                .into_iter()
                .filter(|&c| c != card.color && hand.contains(Card::new(c, Rank::Soldier)))
                .collect();
            for (i, &a) in others.iter().enumerate() {
                for &b in &others[i + 1..] {
                    melds.push(Meld::Soldiers([card.color, a, b].into_iter().collect()));
                }
            }
            if others.len() == 3 {
                melds.push(Meld::Soldiers(ColorSet::ALL));
            }
        }
    }
    melds
}

/// A hand laid out as melds
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// Total value of the melds, minus the cost of the loose cards
    pub score: i32,
    /// The melds, in the order the solver placed them
    pub melds: Vec<Meld>,
    /// Cards left out of every meld; empty for an exact partition
    pub loose: Hand,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Meld(Meld),
    Loose(Card),
}

/// Memoized layout solver for one [`Values`] table
///
/// With `loose` set, every card may be left out of the melds at that cost,
/// so a layout always exists and its score grades an unfinished hand.
/// Without it, only exact partitions count and a hand that cannot be fully
/// laid out has no layout.  The memo persists across calls and is only
/// valid for the table and cost the solver was built with.
#[derive(Debug, Clone)]
pub struct Solver {
    values: Values,
    loose: Option<u32>,
    memo: HashMap<Hand, Option<(i32, Step)>>,
}

impl Solver {
    /// A solver for `values`, leaving cards loose at `loose` each if given
    #[must_use]
    pub fn new(values: Values, loose: Option<u32>) -> Self {
        Self {
            values,
            loose,
            memo: HashMap::new(),
        }
    }

    /// The scoring table
    #[must_use]
    #[inline]
    pub const fn values(&self) -> &Values {
        &self.values
    }

    /// Forget every memoized hand
    pub fn clear(&mut self) {
        self.memo.clear();
    }

    /// The best score of `hand`, if it has a layout
    pub fn best(&mut self, hand: Hand) -> Option<i32> {
        self.solve(hand)
    }

    /// The best layout of `hand`, if it has one
    pub fn layout(&mut self, hand: Hand) -> Option<Layout> {
        let score = self.solve(hand)?;
        let mut layout = Layout {
            score,
            melds: Vec::new(),
            loose: Hand::EMPTY,
        };
        let mut rest = hand;
        while !rest.is_empty() {
            match self.memo[&rest].expect("solved hands memoize a step").1 {
                Step::Meld(meld) => {
                    rest -= meld.cards();
                    layout.melds.push(meld);
                }
                Step::Loose(card) => {
                    rest.remove(card);
                    layout.loose.insert(card);
                }
            }
        }
        Some(layout)
    }

    fn solve(&mut self, hand: Hand) -> Option<i32> {
        let Some(card) = hand.first() else {
            return Some(0);
        };
        if let Some(&memo) = self.memo.get(&hand) {
            return memo.map(|(score, _)| score);
        }
        // The lowest card belongs to some meld or is loose; try each and
        // recurse on the remainder.
        let rest = hand.without(card);
        let mut best: Option<(i32, Step)> = None;
        let mut offer = |score, step| {
            if best.is_none_or(|(b, _)| score > b) {
                best = Some((score, step));
            }
        };
        if let Some(cost) = self.loose
            && let Some(score) = self.solve(rest)
        {
            offer(score - cost as i32, Step::Loose(card));
        }
        for meld in completions(rest, card) {
            if let Some(score) = self.solve(rest - meld.cards().without(card)) {
                offer(score + self.values.score(&meld) as i32, Step::Meld(meld));
            }
        }
        self.memo.insert(hand, best);
        best.map(|(score, _)| score)
    }
}

/// The exact partition of `hand` into melds maximizing `values`, if any
#[must_use]
pub fn partition(hand: Hand, values: Values) -> Option<Layout> {
    Solver::new(values, None).layout(hand)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: Values = Values {
        single: 1,
        pair: 0,
        triplet: 3,
        quad: 8,
        court: 2,
        field: 1,
        three_soldiers: 3,
        four_soldiers: 5,
    };

    fn hand(s: &str) -> Hand {
        s.parse().unwrap()
    }

    fn card(s: &str) -> Card {
        s.parse().unwrap()
    }

    #[test]
    fn completions_list_every_shape() {
        let melds = completions(hand("帥帥仕相..."), card("紅帥"));
        assert_eq!(
            melds,
            [
                Meld::Single(card("紅帥")),
                Meld::Pair(card("紅帥")),
                Meld::Triplet(card("紅帥")),
                Meld::Court(Color::Red),
            ]
        );
        assert_eq!(
            completions(hand("俥炮..."), card("紅傌")),
            [Meld::Field(Color::Red)]
        );
        assert_eq!(completions(hand("俥..."), card("紅傌")), []);

        let soldiers = completions(hand("兵.兵.卒."), card("白卒"));
        assert_eq!(soldiers.len(), 4);
        assert!(soldiers.iter().all(|m| matches!(m, Meld::Soldiers(_))));
        assert_eq!(soldiers[3], Meld::Soldiers(ColorSet::ALL));
        assert_eq!(soldiers[3].cards(), hand("兵.兵.卒.卒"));
        assert_eq!(soldiers[0].size(), 3);
        assert_eq!(completions(hand("兵..."), card("白卒")), []);
    }

    #[test]
    fn partition_maximizes_value() {
        // Three generals: three singles (3) ties a triplet (3); either way 3.
        assert_eq!(partition(hand("帥帥帥..."), VALUES).unwrap().score, 3);
        // A court beats a lone general plus a loose pair: there is no loose.
        let layout = partition(hand("帥仕相..."), VALUES).unwrap();
        assert_eq!(
            layout,
            Layout {
                score: 2,
                melds: vec![Meld::Court(Color::Red)],
                loose: Hand::EMPTY
            }
        );
        assert_eq!(partition(hand("仕相..."), VALUES), None);
        assert_eq!(partition(hand("仕仕相相..."), VALUES).unwrap().score, 0);
        // Four soldiers: one four-colour set (5) beats a three-colour set
        // plus a loose card (impossible exactly) — and the exact layout exists.
        assert_eq!(partition(hand("兵.兵.卒.卒"), VALUES).unwrap().score, 5);
        // Four of each red court piece and three of each yellow one: three
        // concealed quads (24) and three concealed triplets (9) beat seven
        // courts (14).
        let dream = hand("帥帥帥帥仕仕仕仕相相相相.帥帥帥仕仕仕相相相..");
        assert_eq!(dream.len(), 21);
        assert_eq!(partition(dream, VALUES).unwrap().score, 33);
        // Three of each court piece: six concealed triplets (18) beat six courts (12).
        assert_eq!(
            partition(hand("帥帥帥仕仕仕相相相.帥帥帥仕仕仕相相相.."), VALUES)
                .unwrap()
                .score,
            18
        );
        assert_eq!(partition(Hand::EMPTY, VALUES).unwrap().score, 0);
    }

    #[test]
    fn loose_cards_cost() {
        let mut solver = Solver::new(VALUES, Some(1));
        let layout = solver.layout(hand("帥仕.俥..")).unwrap();
        // 帥 single (1), 仕 loose (−1), 俥 loose (−1).
        assert_eq!(layout.score, -1);
        assert_eq!(layout.melds, [Meld::Single(card("紅帥"))]);
        assert_eq!(layout.loose, hand("仕.俥.."));
        // The memo survives: a second query is consistent.
        assert_eq!(solver.best(hand("帥仕.俥..")), Some(-1));
        assert_eq!(solver.best(hand("仕仕...")), Some(0));
    }
}
