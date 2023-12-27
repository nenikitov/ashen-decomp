// Discussion about possible future alternatives:
// https://github.com/rust-lang/rust/pull/101179
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(let_chains)]
#![feature(lazy_cell)]
#![feature(io_error_more)]


pub mod asset;
pub mod directory;
pub mod error;
pub mod utils;
