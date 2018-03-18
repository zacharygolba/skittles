# skittles

Add colors and formatting to ANSI terminal output with easy-to-use macros built on top of [`ansi_term`](https://docs.rs/ansi_term).

## Installation

First, add `skittles` to the dependencies section of your `Cargo.toml`:

```toml
[dependencies]
skittles = "0.1"

```

Next, add add this to the entrypoint of your crate (`lib.rs` or `main.rs`).

```rust
#[macro_use]
extern crate skittles;
```

## Usage

Each macro provided by `skittles` can accept a string literal, an expression that evaluates to a string like value, or a format string with arguments as input.

```rust
println!(
    "{} - {} {} {}.",
    underline!("Skittles"),
    red!("Taste"),
    green!("the"),
    blue!("rainbow")
);
```

You can also compose `skittles` macros together to get the exact color and formatting you want while avoiding itermediate allocations entirely.

```rust
println!(
    "{} - {} {} {}.",
    blink!(yellow!("Skittles")),
    underline!(red!("Taste")),
    underline!(italic!(green!("the"))),
    underline!(bold!(blue!("rainbow")))
);
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
