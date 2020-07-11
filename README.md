(https://crates.io/crates/test-temp-file) ![Rust](https://github.com/ohaddahan/test-temp-file/workflows/Rust/badge.svg)

## [Test Temp File]


This crate allows creation and automatic deletion (based on [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait) of files.

This is aimed mostly for testing purposes, for example when testing a parser you probably
want to read/write file and validate their content
