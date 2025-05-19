# vds

**Visibly Distinguishable Strings** — a `#![no_std]` Rust crate for creating and validating string identifiers composed of easily readable characters.

This crate helps you generate and work with strings that:

- Exclude confusing glyphs like `O`, `0`, `I`, `1`
- Use only uppercase Latin letters and select digits
- Are safe for UI display, voice transmission, QR encoding, and more

---

## 🚀 Getting Started

Install via Cargo:

```sh
cargo add vds
```

Enable optional features:

- `serde` — enables `Serialize`/`Deserialize` for `VDChar` and `VDString`
- `generate` — adds a builder for random string generation using `rand_core`

```toml
[dependencies]
vds = { version = "1.0", features = ["generate", "serde"] }
```

---

## 🎯 Types

### `VDChar`

A single, clearly readable character from a curated set.

```rust
use vds::VDChar;
assert!(VDChar::new('A').is_some());
assert!(VDChar::new('O').is_none()); // O is excluded
```

### `VDString`

A validated string composed of `VDChar`s. Acts like `&str` and supports `.parse()`, indexing, and iteration.

```rust
use vds::VDString;
let code: VDString = "AB29XY".parse().unwrap();
assert_eq!(&*code, "AB29XY");
```

### `VDGenerator` *(requires `generate` feature)*

A builder for generating readable strings with optional constraints:

```rust
use vds::VDGenerator;
use rand::SeedableRng;

let mut rng = rand::rngs::SmallRng::seed_from_u64(123);
let code = VDGenerator::new()
    .length(8)
    .no_repeats()
    .no_adjacent_repeats()
    .generate(&mut rng)
    .unwrap();
```

---

## 🧪 Development & Testing

### 🛠 Dev Commands

This repo includes a `.cargo/config.toml` file with helpful aliases:

- `cargo docs` — build and open full-featured docs
- `cargo tests` — run the full test suite with all features
- `cargo checks` — check build with no default features
- `cargo builds` — build with all features enabled

## 📦 Crate Metadata

- License: MIT OR Apache-2.0
- Repo: [github.com/ianwillis98/vds](https://github.com/ianwillis98/vds)
- Minimum Rust: 1.65
- No `std` required

---

## 📜 License

Licensed under either of:

- MIT OR Apache-2.0
