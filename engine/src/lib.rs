#![warn(clippy::pedantic)]
#![allow(
    unused,
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::wildcard_imports,

    // TODO(Unavailable): Consider reactivating these.
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
)]
#![warn(unused_imports)]
#![feature(
    // Discussion about possible future alternatives:
    // https://github.com/rust-lang/rust/pull/101179
    maybe_uninit_uninit_array_transpose,
    let_chains,
    lazy_cell,
    io_error_more,
    associated_type_defaults,
    trait_alias,
)]

pub mod asset;
pub mod directory;
pub mod error;
mod utils;
