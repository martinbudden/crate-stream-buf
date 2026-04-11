#![doc = include_str!("../README.md")]
#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![warn(unused_results)]
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]

mod stream_buf_reader;
mod stream_buf_writer;

pub use stream_buf_reader::StreamBufReader;
pub use stream_buf_writer::StreamBufWriter;
