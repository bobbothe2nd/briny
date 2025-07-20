# Changelog - `v0.2.0`

`briny v0.2.0` - Zero Trust Made Practical

**Release Date:** 2025-07-20

This is a major improvement in security, restricting usage severely over previous versions.

## General Changes

- Renamed `UntrustedData::value` to `UntrustedData::as_ref`
- Renamed `TrustedData::get` to `TrustedData::as_ref`

## Security Improvements

- Added `UntrustedData::trust` that validates `T`, confirming that the `UntrustedData` can be trusted.
  - The path to `TrustedData` is now more clear

### Deprecated

The following APIs have been removed:

- `UntrustedData::into_inner()`: removed entirely
- Various trait implementations, including:
  - `TrustedData: Clone`
  - `UntrustedData: Debug`
