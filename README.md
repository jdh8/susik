# susik

[![Crates.io](https://img.shields.io/crates/v/susik)](https://crates.io/crates/susik)
[![Docs.rs](https://docs.rs/susik/badge.svg)](https://docs.rs/susik)
[![Build Status](https://github.com/jdh8/susik/actions/workflows/rust.yml/badge.svg)](https://github.com/jdh8/susik)

This crate models the deck of [Four Color Cards] (四色牌, Taiwanese *sù-sik*,
hence the name): 112 narrow cards printing the seven Xiangqi pieces in four
colours, four copies each.  It covers what every game of the deck shares —
colours, pieces, cards, multiset hands, the meld shapes, and a solver that
lays a hand out as melds — and leaves dealing, claims, and scoring to the
game crates, such as [tsap-oo] (十胡, the Taiwanese game).

Red and yellow cards print 帥仕相俥傌炮兵; green and white cards print
將士象車馬包卒.  A card is written as its colour and printed character,
`紅帥`, or as two ASCII letters, `RK` (colour `R Y G W`, piece `K A E R H C P`
after Xiangqi's WXF notation).  A hand is four dot-separated colour groups in
the order red, yellow, green, white: `"帥帥仕.俥.將.卒卒"` holds two red
generals, a red advisor, a yellow chariot, a green general, and two white
soldiers.

[Four Color Cards]: https://en.wikipedia.org/wiki/Four_color_cards
[tsap-oo]: https://github.com/jdh8/tsap-oo

## Modules

- the crate root: [`Color`] and [`Rank`]
- [`hand`][mod-hand]: [`Card`] and the multiset [`Hand`], 28 four-bit
  counts in a `u128`
- [`meld`][mod-meld]: [`Meld`] shapes, [`Values`] tables, [`completions`]
  (what a card completes), and the memoized [`Solver`] behind [`partition`];
  `Solver::locked` and `locked_completions` keep three or four identical
  cards whole, for the games that forbid splitting them
- [`deck`][mod-deck] (feature `rand`): [`shuffle`]

## Feature flags

- `rand`: shuffling the deck

## Quick start

```rust
use susik::{Hand, Meld, Values, partition};

// 十胡's values for concealed melds.
let values = Values {
    single: 1, pair: 0, triplet: 3, quad: 8,
    court: 2, field: 1, three_soldiers: 3, four_soldiers: 5,
};
let hand: Hand = "帥仕相.俥傌炮..".parse()?;
let layout = partition(hand, values).expect("a court and a field");
assert_eq!(layout.score, 3);
assert_eq!(layout.melds, [Meld::Court(susik::Color::Red), Meld::Field(susik::Color::Yellow)]);
# Ok::<(), susik::ParseHandError>(())
```

[mod-hand]: https://docs.rs/susik/latest/susik/hand/index.html
[mod-meld]: https://docs.rs/susik/latest/susik/meld/index.html
[mod-deck]: https://docs.rs/susik/latest/susik/deck/index.html
[`Color`]: https://docs.rs/susik/latest/susik/enum.Color.html
[`Rank`]: https://docs.rs/susik/latest/susik/enum.Rank.html
[`Card`]: https://docs.rs/susik/latest/susik/struct.Card.html
[`Hand`]: https://docs.rs/susik/latest/susik/struct.Hand.html
[`Meld`]: https://docs.rs/susik/latest/susik/enum.Meld.html
[`Values`]: https://docs.rs/susik/latest/susik/struct.Values.html
[`Solver`]: https://docs.rs/susik/latest/susik/struct.Solver.html
[`completions`]: https://docs.rs/susik/latest/susik/fn.completions.html
[`partition`]: https://docs.rs/susik/latest/susik/fn.partition.html
[`shuffle`]: https://docs.rs/susik/latest/susik/fn.shuffle.html
