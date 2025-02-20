// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![doc = include_str!("./../README.md")]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc,
  // clippy::missing_errors_doc
)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

pub use self::error::Error;
pub use self::error::Result;

#[cfg(feature = "credential")]
pub mod credential;
pub mod error;
#[cfg(feature = "presentation")]
pub mod presentation;
#[cfg(feature = "revocation-bitmap")]
pub mod revocation;

#[cfg(feature = "validator")]
pub mod validator;
