# `small-unicode`, a tiny, non-compliant, reimplementation of [`unicode` by Radovan Garabík (on APT: Debian/Ubuntu/Mint)](https://github.com/garabik/unicode)

![shield: made in rust](https://img.shields.io/badge/Made%20in%20Rust-orange?logo=rust)

## Why

The original program is made in Python. Each invocation takes 44.6ms on my rather powerful computer, much more on a VPS where I need it for education-related stuff. This version took 20 minutes to make, is exactly tailored to my use case, and I'm using it as a drop-in replacement for the original. One execution now take 1.3ms. Nice.

Yes, I should probably not depend on a command-line tool from code in the first place, and simply use a library, but I don't have the choice due to the exercise's requirements. Hosting our projects on a VPS is definitely not expected from us, and I did it for the sole fun of using continuous integration.

## Installation/building & Usage

```bash
cargo install --git https://github.com/edgarogh/small-unicode
# executable available globally
```

Or, after cloning:

```bash
cargo build --release
# executable in target/release/small-unicode
```

The only supported command in: `small-unicode -d 65 --brief`

## Acknowledgment & license

The software is distributed under GPL-3.0. It probably will never be maintained further.

Huge thanks to [@garabik](https://github.com/garabik) for making the original software that I nastily copied. Use his version, it is much more complete.
