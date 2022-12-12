//! [![github]](https://github.com/sbailleul/mapper)&ensp;[![crates-io]](https://crates.io/crates/mapper)&ensp;[![docs-rs]](https://docs.rs/mapper)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//! This library provides a convenient derive macro for implementing [mapper_api::Mapper<T>] trait.
pub use mapper_impl::*;
pub use mapper_api::*;

#[cfg(test)]
mod tests;