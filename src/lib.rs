#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[cfg(feature = "rand")]
pub mod deck;
pub mod hand;
pub mod meld;

#[cfg(feature = "rand")]
pub use deck::shuffle;
pub use hand::{Card, Hand, ParseCardError, ParseHandError};
pub use meld::{ColorSet, Layout, Meld, Solver, Values, completions, partition};

use core::fmt;
use thiserror::Error;

/// Error for a character that names no colour
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Hash)]
#[error("{0:?} is not a colour: expected one of 紅 黃 綠 白 R Y G W")]
pub struct InvalidColor(pub char);

/// Error for a character that names no piece
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Hash)]
#[error("{0:?} is not a piece: expected one of 將士象車馬包卒 帥仕相俥傌炮兵 K A E R H C P")]
pub struct InvalidRank(pub char);

/// A colour of the deck: the dye of the card stock
///
/// Red and yellow cards print the red Xiangqi pieces 帥仕相俥傌炮兵; green
/// and white cards print the black pieces 將士象車馬包卒.  The derived order
/// is the encoding and display order of this crate, not a ranking: the
/// games treat colours as equals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Color {
    /// 紅
    Red,
    /// 黃
    Yellow,
    /// 綠
    Green,
    /// 白
    White,
}

impl Color {
    /// Colours in the order of this crate
    pub const ALL: [Self; 4] = [Self::Red, Self::Yellow, Self::Green, Self::White];

    /// Uppercase ASCII letter
    #[must_use]
    #[inline]
    pub const fn letter(self) -> char {
        match self {
            Self::Red => 'R',
            Self::Yellow => 'Y',
            Self::Green => 'G',
            Self::White => 'W',
        }
    }

    /// Chinese name
    #[must_use]
    #[inline]
    pub const fn name(self) -> char {
        match self {
            Self::Red => '紅',
            Self::Yellow => '黃',
            Self::Green => '綠',
            Self::White => '白',
        }
    }

    /// The pieces printed on this colour, in [`Rank`] order
    #[must_use]
    #[inline]
    pub const fn pieces(self) -> &'static [char; 7] {
        match self {
            Self::Red | Self::Yellow => &['帥', '仕', '相', '俥', '傌', '炮', '兵'],
            Self::Green | Self::White => &['將', '士', '象', '車', '馬', '包', '卒'],
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name().encode_utf8(&mut [0; 4]))
    }
}

impl TryFrom<char> for Color {
    type Error = InvalidColor;

    /// Accepts the Chinese name or the ASCII letter in either case
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c.to_ascii_uppercase() {
            '紅' | 'R' => Self::Red,
            '黃' | 'Y' => Self::Yellow,
            '綠' | 'G' => Self::Green,
            '白' | 'W' => Self::White,
            _ => return Err(InvalidColor(c)),
        })
    }
}

/// A piece, the face printed on a card
///
/// The seven Xiangqi pieces in their customary listing order, general first.
/// As with [`Color`], the derived order only fixes the encoding of this
/// crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    /// 將 / 帥
    General,
    /// 士 / 仕
    Advisor,
    /// 象 / 相
    Elephant,
    /// 車 / 俥
    Chariot,
    /// 馬 / 傌
    Horse,
    /// 包 / 炮
    Cannon,
    /// 卒 / 兵
    Soldier,
}

impl Rank {
    /// Pieces in the order of this crate
    pub const ALL: [Self; 7] = [
        Self::General,
        Self::Advisor,
        Self::Elephant,
        Self::Chariot,
        Self::Horse,
        Self::Cannon,
        Self::Soldier,
    ];

    /// Uppercase ASCII letter, the WXF notation of Xiangqi
    #[must_use]
    #[inline]
    pub const fn letter(self) -> char {
        match self {
            Self::General => 'K',
            Self::Advisor => 'A',
            Self::Elephant => 'E',
            Self::Chariot => 'R',
            Self::Horse => 'H',
            Self::Cannon => 'C',
            Self::Soldier => 'P',
        }
    }

    /// The character printed for this piece on cards of `color`
    #[must_use]
    #[inline]
    pub const fn glyph(self, color: Color) -> char {
        color.pieces()[self as usize]
    }
}

impl TryFrom<char> for Rank {
    type Error = InvalidRank;

    /// Accepts the character of either piece set (plus the common variant
    /// 砲) or the ASCII letter in either case
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c.to_ascii_uppercase() {
            '將' | '帥' | 'K' => Self::General,
            '士' | '仕' | 'A' => Self::Advisor,
            '象' | '相' | 'E' => Self::Elephant,
            '車' | '俥' | 'R' => Self::Chariot,
            '馬' | '傌' | 'H' => Self::Horse,
            '包' | '炮' | '砲' | 'C' => Self::Cannon,
            '卒' | '兵' | 'P' => Self::Soldier,
            _ => return Err(InvalidRank(c)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyphs_follow_the_colour() {
        assert_eq!(Rank::General.glyph(Color::Red), '帥');
        assert_eq!(Rank::General.glyph(Color::Yellow), '帥');
        assert_eq!(Rank::General.glyph(Color::Green), '將');
        assert_eq!(Rank::Soldier.glyph(Color::White), '卒');
        for color in Color::ALL {
            for rank in Rank::ALL {
                assert_eq!(Rank::try_from(rank.glyph(color)), Ok(rank));
                assert_eq!(Rank::try_from(rank.letter()), Ok(rank));
            }
            assert_eq!(Color::try_from(color.name()), Ok(color));
            assert_eq!(
                Color::try_from(color.letter().to_ascii_lowercase()),
                Ok(color)
            );
        }
        assert_eq!(Color::try_from('X'), Err(InvalidColor('X')));
        assert_eq!(Rank::try_from('X'), Err(InvalidRank('X')));
    }
}
