# Helper

[![Rust](https://github.com/jmjoy/helper/actions/workflows/rust.yml/badge.svg)](https://github.com/jmjoy/helper/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/helper.svg)](https://crates.io/crates/helper)
[![API](https://docs.rs/helper/badge.svg)](https://docs.rs/helper)

A library provided some useful proc macros for Rust.

## Macros

- **control flow**

  - [**either**](https://docs.rs/helper/latest/helper/macro.either.html): Ternary operator in many C-like languages.
  - [**try_option**](https://docs.rs/helper/latest/helper/macro.try_option.html): Unwrap `std::option::Option`, if none, return the alternative value.

- **collections**

  - [**btmap**](https://docs.rs/helper/latest/helper/macro.btmap.html): Create `std::collections::BTreeMap` from list of key-value pairs.
  - [**btset**](https://docs.rs/helper/latest/helper/macro.btset.html): Create a `std::collections::BTreeSet` from a list of elements.
  - [**hmap**](https://docs.rs/helper/latest/helper/macro.hmap.html): Create `std::collections::HashMap` from list of key-value pairs.
  - [**hset**](https://docs.rs/helper/latest/helper/macro.hset.html): Create a `std::collections::HashSet` from a list of elements.

## License

MulanPSL-2.0
