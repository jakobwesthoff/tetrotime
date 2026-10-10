---
title: "Enable a --version flag"
kind: feature
component: app
---
# Enable a --version flag

`tetrotime --version` does not exist, so the binary cannot report its
version. The planned Homebrew formula in `jakobwesthoff/homebrew-tap`
needs that flag to check the installed binary, so it is the fix that
unblocks the formula's test.

## Goal

`tetrotime --version` prints the version of the installed binary.

## Proposal

The clap attribute on `Args` in `src/main.rs` sets `author` and `about`
but not `version`, and clap only adds `-V`/`--version` when `version` is
set:

```rust
#[command(
    author = "Jakob Westhoff <jakob@westhoffswelt.de>",
    about = "TetroTime - Time meets Tetris!"
)]
```

Add `version` to that attribute. Without a value, clap takes the
version from `Cargo.toml` (`CARGO_PKG_VERSION`, currently 1.0.2), so
`tetrotime --version` prints `tetrotime 1.0.2`.

## Why it matters

The planned Homebrew formula in `jakobwesthoff/homebrew-tap` checks the
installed binary in its `test do` block. Every other tool there uses
`--version` for that, matched against the formula's version. Until a
release has the flag, the TetroTime formula can only check `--help` for
the about text. Once a release with the flag exists, switch the
formula's test to `--version`.
