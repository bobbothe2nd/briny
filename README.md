# `briny`

`briny` is a small, secure crate that prevents many memory bugs with a safe API - sometimes it can help prevent undefined behavior at compile time.

## Overview

### `briny` `v0.3.0` is a complete rewrite

This crate no longer enforces Zero Trust Architecture via Rust's type system.
It is now more focused on safe synchronization and memory usage (Although some traits like `Validate` still exist in improved forms...).

`briny` is a low-level Rust crate focused on **safe memory handling in `no_std` environments**.

`v0.3.0` is a **complete rewrite** over `v0.2.0`, shifting its primary intent from validating external input to **preventing internal memory hazards**.  
This release emphasizes safety at the systems level - making data races, undefined behavior, and unsafe casting much harder to introduce accidentally.

## Core Philosophy

briny encapsulates the minimal necessary use of `unsafe` to provide:

- **Thread-safe shared ownership** without relying on the standard library.
- **Controlled interior mutability** without the pitfalls of raw `UnsafeCell`.
- **Safe, validated type casting** for both sized and unsized types.
- **Portable, allocation-agnostic primitives** suitable for embedded, kernel, and bare-metal environments.

The crate avoids over-engineered abstractions, prioritizing **direct, minimal, and predictable primitives** with well-defined safety boundaries.

## Key Architectural Additions

### Allocation-Free Thread-Safe References

`Darc` (Direct ARC) and `Naarc` (Non-Alloc ARC) offer reference counting for shared data **without requiring heap allocation** and without using `std`.  
Both are designed to be `Send + Sync` when the underlying type permits, enabling multi-threaded designs in `no_std` or constrained environments.
Due to their inherent simplicity, they can usually perform better than `Arc`.

### Safer Interior Mutability

`NotUnsafeCell` replaces ad-hoc `UnsafeCell` usage with a structured, safety-checked API.  
It preserves the flexibility of interior mutability while significantly reducing the risk of unsound aliasing or reentrant access.

### Safe Casting for Sized and Unsized Types

`briny` provides casting utilities that:

- Validate alignment, size, and bit-pattern safety before reinterpretation.
- Support unsized types, including slices and trait objects.
- Eliminate undefined behavior from unchecked `transmute` usage.

## Design Goals

- **`no_std` First** - Fully functional without the standard library, with optional `alloc` support.
- **Safety-Oriented** - Encapsulate `unsafe` so it is not spread throughout downstream code.
- **Thread-Aware** - Provide concurrency primitives for systems without `std::sync`.
- **Fuzz-Resistant** - Handle edge-case and malformed inputs gracefully at the memory level.

## Project Status

- Security-first API design
- 100% safe Rust (no unsafe)
- Fully tested (integration and unit tests)
- No dependencies
- `#![no_std]` support
- Not dependent on `alloc` or `std`
- Community audits welcome

## Contributing

Contributions, bug reports, and suggestions are welcome! This project aims to help build verifiably secure foundations for low-level and embedded Rust development.

### License

`briny` is under an MIT license.
