//! Common imports for working with briny securely and ergonomically.
//!
//! This prelude gives you everything needed to use `briny` safely:
//!
//! - [`UntrustedData`] and [`TrustedData`]
//! - [`Validate`], [`Trusted`], and [`Untrusted`] traits
//! - [`Pack`] and [`Unpack`] for binary I/O
//! - [`PackRef`] and [`UnpackBuf`] buffer wrappers
//! - [`ByteBuf`] for raw byte wrapping/parsing
//!
//! Import this prelude to get zero-trust safety with less boilerplate:
//!
//! ```rust
//! use briny::prelude::*;
//! ```

pub use crate::{
    trust::{
        TrustedData, Validate, ValidationError,
        Untrusted, UntrustedData,
    },
    pack::{
        Pack, PackRef, Unpack, UnpackBuf,
    },
    raw::{
        Raw, ByteBuf,
    },
};
pub use core::str::FromStr;