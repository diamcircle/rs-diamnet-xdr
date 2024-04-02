#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docs, feature(doc_auto_cfg))]
// TODO: Remove these clippy doc comment allows after improving the
// auto-generated docs.
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_markdown)]

//! Library and CLI containing types and functionality for working with diamcircle
//! XDR.
//!
//! Types are generated from XDR definitions hosted at [diamcircle/diamcircle-xdr]
//! using [xdrgen].
//!
//! [diamcircle/diamcircle-xdr]: https://github.com/diamcircle/diamcircle-xdr
//! [xdrgen]: https://github.com/diamcircle/xdrgen
//!
//! ## Usage
//!
//! ### Library
//! To use the library, include in your toml:
//!
//! ```toml
//! diamcircle-xdr = { version = "...", default-features = true, features = [] }
//! ```
//!
//! #### Features
//!
//! The crate has several features, tiers of functionality, ancillary
//! functionality, and channels of XDR.
//!
//! Default features: `std`, `curr`.
//!
//! Teirs of functionality:
//!
//! 1. `std` – The std feature provides all functionality (types, encode,
//! decode), and is the default feature set.
//! 2. `alloc` – The alloc feature uses `Box` and `Vec` types for recursive
//! references and arrays, and is automatically enabled if the std feature is
//! enabled. The default global allocator is used. Support for a custom
//! allocator will be added in [#39]. No encode or decode capability exists,
//! only types. Encode and decode capability will be added in [#46].
//! 3. If std or alloc are not enabled recursive and array types requires static
//! lifetime values. No encode or decode capability exists. Encode and decode
//! capability will be added in [#47].
//!
//! [#39]: https://github.com/diamcircle/rs-diamcircle-xdr/issues/39
//! [#46]: https://github.com/diamcircle/rs-diamcircle-xdr/issues/46
//! [#47]: https://github.com/diamcircle/rs-diamcircle-xdr/issues/47
//!
//! Ancillary functionality:
//!
//! 1. `base64` – Enables support for base64 encoding and decoding.
//! 2. `serde` – Enables support for serializing and deserializing types with
//! the serde crate.
//! 3. `arbitrary` – Enables support for interop with the arbitrary crate.
//!
//! Channels of XDR:
//!
//! - `curr` – XDR types built from the `diamcircle/diamcircle-xdr` `curr` branch.
//! - `next` – XDR types built from the `diamcircle/diamcircle-xdr` `next` branch.
//!
//! If a single channel is enabled the types are available at the root of the
//! crate. If multiple channels are enabled they are available in modules at
//! the root of the crate.
//!
//! ### CLI
//!
//! To use the CLI:
//!
//! ```console
//! cargo install --locked diamcircle-xdr --version ... --features cli
//! ```
//!
//! #### Examples
//!
//! Parse a `TransactionEnvelope`:
//! ```console
//! diamcircle-xdr decode --type TransactionEnvelope << -
//! AAAAA...
//! -
//! ```
//!
//! Parse a `ScSpecEntry` stream from a contract:
//! ```console
//! diamcircle-xdr +next decode --type ScSpecEntry --input stream-base64 --output json-formatted << -
//! AAAAA...
//! -
//! ```
//!
//! Parse a `BucketEntry` framed stream from a bucket file:
//! ```console
//! diamcircle-xdr decode --type BucketEntry --input stream-framed --output json-formatted bucket.xdr
//! ```

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version<'a> {
    pub pkg: &'a str,
    pub rev: &'a str,
    pub xdr: &'a str,
    pub xdr_curr: &'a str,
    pub xdr_next: &'a str,
}
pub const VERSION: Version = Version {
    pkg: env!("CARGO_PKG_VERSION"),
    rev: env!("GIT_REVISION"),
    xdr: if cfg!(all(feature = "curr", feature = "next")) {
        "curr,next"
    } else if cfg!(feature = "curr") {
        "curr"
    } else if cfg!(feature = "next") {
        "next"
    } else {
        ""
    },
    xdr_curr: include_str!("../xdr/curr-version"),
    xdr_next: include_str!("../xdr/next-version"),
};

#[cfg(feature = "curr")]
pub mod curr;

#[cfg(feature = "next")]
pub mod next;

#[cfg(feature = "cli")]
pub mod cli;
