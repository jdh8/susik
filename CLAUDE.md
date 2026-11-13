# susik

The deck of Four Color Cards (四色牌) and the rules every game of it shares:
cards, multiset hands, meld shapes, and the layout solver.  Dealing, claims,
turn order, and scoring belong to game crates such as `../tsap-oo` (十胡).

After updating the codebase, please

- Format the code with `cargo fmt`.
- Lint with `cargo clippy --all-targets --all-features -- -D warnings`.
- Run the tests with `cargo test --all-features`, then `cargo test` without
  features (the README doctest must compile both ways).
- Build the docs with `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`.
- Note user-visible changes in `CHANGELOG.md` under `[Unreleased]`.
- Propose a clear and descriptive commit message.

House style: derive order `Debug, Clone, Copy, PartialEq, Eq, PartialOrd,
Ord, Hash`; `#[must_use]`, `#[inline]`, and `const fn` on small methods; docs
are prose with two spaces after sentence-ending periods and no trailing
period on item summaries.  The `u128` layout of `Hand` is a documented,
stable API.
