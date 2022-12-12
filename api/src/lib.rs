/*!
[![github]](https://github.com/sbailleul/mapper)&ensp;[![crates-io]](https://crates.io/crates/mapper)&ensp;[![docs-rs]](https://docs.rs/mapper)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
This library provide definitions used by [mapper](https://docs.rs/mapper-api/1.0.1/mapper_api) crate
 */


/// Trait defining a mapper converting itself to a destination Type of T 
pub trait Mapper<T>{
    fn to(&self)->T;
}

