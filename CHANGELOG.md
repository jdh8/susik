# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-15

### Added

- `Solver::locked` and `locked_completions`: keep three or four identical
  cards in a hand whole, as nearly every game of the deck requires, so
  車車車馬包 is a triplet with 馬包 loose rather than 車車 and 車馬包.

## [0.1.0] - 2026-11-14

### Added

- `Color`, `Rank`, `Card`: the 112-card deck with its colour-dependent
  printed characters, parsed from `紅帥` or `RK`.
- `Hand`: a multiset of cards packed as 28 nibbles in a `u128`, written as
  four dot-separated colour groups.
- `Meld`, `Values`, `completions`, `Solver`, `partition`: the meld shapes
  shared by the games, and a memoized solver laying a hand out for the most
  value, exactly or with loose cards at a cost.
- `shuffle` (feature `rand`).
