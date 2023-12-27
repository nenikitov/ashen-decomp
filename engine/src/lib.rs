#![allow(
    unused,
    clippy::cast_lossless,
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]
#![warn(clippy::pedantic, unused_imports)]
#![feature(
    // Discussion about possible future alternatives:
    // https://github.com/rust-lang/rust/pull/101179
    maybe_uninit_uninit_array_transpose,
    let_chains,
    lazy_cell,
    io_error_more
)]

pub mod asset;
pub mod directory;
pub mod error;
pub mod utils;
