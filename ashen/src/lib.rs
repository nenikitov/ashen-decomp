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
    clippy::cast_sign_loss,
    clippy::missing_fields_in_debug,
    clippy::unreadable_literal,

    incomplete_features,
)]
#![warn(unused_imports)]
#![feature(
    debug_closure_helpers,
    generic_const_exprs,
    io_error_more,
    let_chains,
    // Discussion about possible future alternatives:
    // https://github.com/rust-lang/rust/pull/101179
    maybe_uninit_uninit_array_transpose,
)]

pub mod asset;
pub mod asset_binrw;
pub mod directory;
pub mod error;
mod utils;
