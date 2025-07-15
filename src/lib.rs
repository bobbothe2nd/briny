//! # briny
//!
//! A zero-unsafe, `#![no_std]` library for securely handling untrusted binary data.
//!
//! `briny` enforces strict trust boundaries between unvalidated input and
//! verified, structured data. Its design ensures that parsing, validation, and
//! access are **explicit**, **safe**, and **clear at the type level**.
//!
//! ## Core Principles
//!
//! - **No unsafe code**: Guaranteed by `#![forbid(unsafe_code)]`.
//! - **No allocations**: Fully compatible with `#![no_std]` environments.
//! - **Trust boundaries encoded in types**: Untrusted data must be validated before use.
//! - **Composable parsing and serialization**: via `Raw`, `Pack`, `Unpack`, etc.
//! - **No dependencies**: Pure core-based abstractions for maximum auditability.
//!
//! ## Modules
//!
//! - [`trust`](crate::trust): Core traits and types for enforcing trust separation.
//! - [`raw`](crate::raw): Generic fixed-size byte containers for raw binary fields.
//! - [`pack`](crate::pack): Binary packing and unpacking with validation gates.
//! - [`prelude`](crate::prelude): Convenient re-exports for core traits.
//!
//! ## Example: Validating and Unpacking Raw Data
//!
//! ```rust
//! use briny::prelude::*;
//!
//! #[derive(Debug)]
//! struct MyData(u32);
//!
//! impl Validate for MyData {
//!     fn validate(&self) -> Result<(), ValidationError> {
//!         if self.0 < 100 { Ok(()) } else { Err(ValidationError) }
//!     }
//! }
//!
//! impl Raw<4> for MyData {
//!     fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
//!         Ok(MyData(u32::from_le_bytes(bytes)))
//!     }
//!
//!     fn to_bytes(&self) -> [u8; 4] {
//!         self.0.to_le_bytes()
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let raw = ByteBuf::<MyData, 4>::new(42u32.to_le_bytes());
//!     let parsed = raw.parse()?; // parsed is MyData
//!     let trusted = TrustedData::new(parsed)?; // now this works, because MyData: Validate
//!     Ok(())
//! }
//! ```
//!
//! ### Reccomendations
//!
//! The prelude module contains most items and it is reccomended, when using many
//! features, to begin a crate with the following:
//!
//! ```rust
//! use briny::prelude::*;
//! ```
//!
//! This is especially important when using `ByteBuf::from_str()` which requires the
//! trait `core::str::FromStr` to be properly utilized.
//!
//! ## Who Should Use This?
//!
//! - Embedded developers parsing network/protocol data
//! - Cryptographic libraries enforcing strict validation before processing
//! - Any `no_std` crate with a hard requirement on memory safety and correctness
//!
//! ## Feature Goals
//!
//! - [X] No unsafe
//! - [X] No dependencies
//! - [X] No allocation
//! - [ ] Support endian-specific parsing (`BigEndian`, `LittleEndian`)
//! - [ ] Derive macros for `Raw` and `Validate` (via optional proc-macro crate)
//!
//! ## Security Note
//!
//! This crate is intentionally restrictive. If something feels tedious, it's likely
//! because the operation requires *explicit trust handling*. The goal is to force
//! thoughtful design, especially in security-critical environments.
//!
//! ## Binary size
//! `briny` avoids heap allocation, formatting macros, or panicking branches.
//! Its design aims to support limited embedded targets (e.g. 32 KiB MCUs).

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]

pub mod pack;
pub mod prelude;
pub mod raw;
pub mod trust;
