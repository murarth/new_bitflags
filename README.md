# Deprecated

This crate was written before the introduction of associated constants,
when the [`bitflags`](https://crates.io/crates/bitflags) crate would generate
a module containing constant items for defined flags. Those were dark times.

However, now that associated constants are here, `bitflags` is wonderfully
ergonomic and this crate is obsolete.

Therefore, feel free to disregard this crate and use `bitflags` instead.

We now return to your regularly scheduled README:

# `new_bitflags`

More ergonomic bitflags

[Documentation](https://docs.rs/new_bitflags/)

## Building

To include `new_bitflags` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
new_bitflags = "0.1"
```

And the following to your crate root:

```rust
#[macro_use] extern crate new_bitflags;
```

## License

`new_bitflags` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
