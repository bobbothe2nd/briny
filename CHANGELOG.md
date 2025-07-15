# Changelog - `v0.1.1`

`briny v0.1.1` - Zero Trust Made Practical

**Release Date:** 2025-07-15

This is a minor but meaningful upgrade focused on usability, security edge case handling, and long-term structure for Zero Trust Architecture (ZTA) adoption. It introduces non-breaking enhancements while paving the way for a hardened `v0.2.0`.

## Added

### Trust System

- `Validate<C>` now supports contextual validation via `validate_with(&C)` (default context: `()`).

### `ByteBuf` API

- `.as_untrusted<T>()`: Projects a buffer as `UntrustedData<T>`.
- `.try_unpack<T>()`: Unpacks and validates directly from a raw buffer.
- `.chunks<T>()`: Produces a fallible iterator over untrusted `T`-sized chunks.

## Changed

- Made `TrustedData::new()` more clearly documented as the only path to trust elevation.
- Rewrote internal panics to use `ValidationError` instead of `unwrap()` where relevant.
- Added `#[inline(always)]` to core hot paths in `trust` and `pack` modules for LTO friendliness.

## Security Notes

- All trust transitions are now auditable and strictly typed.
- `TrustedData` can no longer be constructed externally due to sealing.
- Validation logic now supports *context-aware* decisions (`Validate<C>`), allowing dynamic ZTA policies.
- Trust-state violations are caught early in tests with panic-free `ValidationError`s.
- Unsafe trait markers (e.g., `RawSafe`) are opt-in only and clearly documented.

## Plans for v0.2.0

- Make `.into_inner()` and `.get()` safer or remove them entirely.
- Enforce `RawSafe + Validate<C>` at the type level on all `TrustedData<T>`.
- Add `TrustedRef<'a, T>` and `UntrustedRef<'a, T>` for capability-based access.
- Make `TrustedData` non-Clone unless `T: RawSafe`.

### Deprecated (for `v0.2.0`)

The following APIs are still available in `v0.1.1`, but are marked for removal in the next major release:

- `TrustedData::get()`: Will be replaced with `.as_ref()` or `TrustedRef<T>`.
- `UntrustedData::into_inner()`: Will be removed to enforce validation-first workflows.
- `Validate::validate()`: Will be replaced by `validate_with(&C)` as the canonical path.
- `TrustedData: Clone`: Will be gated by `T: RawSafe` or removed entirely.
