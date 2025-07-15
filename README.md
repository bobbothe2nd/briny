# briny

`briny` is one of the only Rust crates that enforces binary trust boundaries at compile time — zero unsafe, no-alloc, no-macro.

`briny` gives you airtight control over what data is trusted and when. It helps you securely parse, validate, and serialize binary-structured data without ever trusting unchecked input.

## What Makes `briny` Different?

`briny` enforces Zero Trust Architecture (ZTA) principles at compile time. Just like Rust's ownership system prevents memory safety bugs before runtime, `briny` prevents logic from touching untrusted or unvalidated input. No hopeful parsing, no runtime footguns.

### If you follow `briny`'s rules

- All external data must pass `Validate` before use
- All deserialized structures are wrapped in `TrustedData`
- No logic can access unchecked input without being explicit

### If you *don't* follow the rules

- It's like misusing `unsafe {}` — *you opt out of the safety net*
- `briny` won't stop you from writing broken or insecure `Validate` impls
- You can still violate trust boundaries *after* validation, if you ignore discipline
- `briny` can't enforce runtime misuse beyond the type system.

## Why Use `briny`?

- Enforce trust boundaries with marker traits (`Trusted`, `Untrusted`)
- Zero dependencies and `#![no_std]` compatible - no `alloc` either
- Built for embedded, security-critical, and sandboxed Rust systems
- Prevent bugs before you even test for them

### Warning: `briny` Is a Power Tool

`briny` helps you build airtight validation infrastructure — but like `unsafe`, it must be used with care. If downstream crates implement `Validate` incorrectly, or mutate `TrustedData` unsafely, no compile-time model can save you.

Use `briny` to make the safe path easy and the unsafe path obvious.

`briny` is ideal for:

- Hardened OS modules
- Secure microservices
- Kernel/user-mode message passing
- Cryptographic protocols
- Embedded bootloaders and firmware parsing
- WASM interfaces

Use `briny` where trust boundaries matter most — and test isn't enough.

## Features

- Zero Trust Architecture (ZTA)–aligned
- Binary-safe serialization via `Pack`/`Unpack`
- Trusted vs. untrusted data split at the type level
- Fixed-size byte buffer abstraction (`ByteBuf<T, N>`)
- Dependency-free (no `std` or `alloc`)

### Trust Model

`briny` enforces explicit trust boundaries using:

- `UntrustedData<T>`: Marker for unsafe input (from users, network, disk, etc.)
- `Validate`: Trait that defines the rules for converting untrusted data into trusted form
- `TrustedData<T>`: Guarantees validation has occurred — only safe data gets in
- Sealed trait `Trusted` ensures trust cannot be used outside the crate

This ZTA-style model improves security on many frontiers, meaning:

- No unchecked logic runs on untrusted data
- All transitions are explicit and type-checked
- You *can't* forget to validate

## Example

```rust
use briny::prelude::*;

struct MyData([u8; 4]);

impl Validate for MyData {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0[0] == 42 { Ok(()) } else { Err(ValidationError) }
    }
}

impl Pack for MyData {
    fn pack(&self, mut out: PackRef<'_>) -> Result<(), ValidationError> {
        out.ref_mut().copy_from_slice(&self.0);
        Ok(())
    }
}

impl Unpack for MyData {
    fn unpack_and_validate(input: UnpackBuf<'_>) -> Result<TrustedData<'_, Self>, ValidationError> {
        let slice = input.as_slice();
        if slice.len() != 4 {
            return Err(ValidationError);
        }
        let data = MyData([slice[0], slice[1], slice[2], slice[3]]);
        TrustedData::new(data)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // random bytes
    let external = UntrustedData::new([42, 0, 0, 0]);

    // validate the payload
    let my = MyData(external.into_inner());
    let trusted = TrustedData::new(my)?;

    // serialize it
    let mut buf = [0u8; 4];
    trusted.pack(PackRef::new(&mut buf))?;
    assert_eq!(buf, [42, 0, 0, 0]);
    
    Ok(())
}
```

## Comparison

| Feature | `serde` | `validator` | `nom` | `briny` |
|-----------------------------------|:-:|:-:|:-:|:-:|
| Compile-time security guarantees  | N | N | N | Y |
| Blocks parsing before validation  | N | N | N | Y |
| Validation enforced by compiler   | N | N | N | Y |
| Type-level trust separation       | N | N | N | Y |
| `no_std` compatible               | ~ | N | Y | Y |
| Accidental bypasses impossible    | N | N | N | Y |

### What `briny` Does Not Do

While `briny` enforces trust boundaries at compile time, it's not a one-size-fits-all validation framework. It doesn't...

- Parse or validate complex or nested data formats like JSON, XML, or YAML.
- Handle cryptographic operations or key management.
- Provide runtime-configurable validation rules or dynamic schema updates.
- Offer detailed validation error reporting with rich diagnostics.
- Support heap allocations or complex data structures requiring `std` or `alloc`.

Use `briny` when you need binary-safe, zero-cost, compile-time enforced trust for fixed-layout, embedded, or low-level data structures.

For everything else—especially rich data formats or dynamic validation—consider combining `briny` with crates like `serde`, `validator`, or `nom`.

Here is a comparison between `briny`, `serde`, `validator`, and `nom` where each must validate a 4-byte array where the first byte is 42.

#### serde — Deserialization without enforced validation

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct MyData([u8; 4]);

// Deserialize blindly, even if data is bad
let my: MyData = bincode::deserialize(&input_bytes)?;

// No guarantee this is safe!
assert_eq!(my.0[0], 42); // Could panic or be wrong
```

Risk: Data is used before it's validated. The deserialized value is implicitly trusted.

#### validator — Runtime validation, trust still implicit

```rust
use validator::{Validate};

#[derive(Validate)]
struct MyData {
    #[validate(custom = "validate_first_byte")]
    data: [u8; 4],
}

fn validate_first_byte(data: &[u8; 4]) -> Result<(), validator::ValidationError> {
    if data[0] == 42 { Ok(()) } else { Err(ValidationError::new("bad")) }
}

// Data is deserialized before it's validated
let my: MyData = serde_json::from_str(json_input)?;
my.validate()?; // You must remember to call this!
```

Risk: Forgeting to call `.validate()` could end horribly, everything is runtime-based.

#### nom — Binary parsing with separate validation

```rust
use nom::{bytes::complete::take, IResult};

fn parse(input: &[u8]) -> IResult<&[u8], [u8; 4]> {
    let (rest, bytes) = take(4usize)(input)?;
    Ok((rest, [bytes[0], bytes[1], bytes[2], bytes[3]]))
}

// Parse succeeds regardless of content
let (_, my_data) = parse(&input_bytes)?;
if my_data[0] != 42 {
    return Err("bad");
}
```

Risk: Parsing and validation are disconnected. It's easy to skip checks.

#### briny — Enforced trust boundaries at the type level

```rust
use briny::prelude::*;

struct MyData([u8; 4]);

impl Validate for MyData {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0[0] == 42 { Ok(()) } else { Err(ValidationError) }
    }
}

impl Unpack for MyData {
    fn unpack_and_validate(input: UnpackBuf<'_>) -> Result<TrustedData<'_, Self>, ValidationError> {
        let slice = input.as_slice();
        if slice.len() != 4 { return Err(ValidationError); }
        TrustedData::new(MyData([slice[0], slice[1], slice[2], slice[3]]))
    }
}

// From raw input to trusted, validated value
let buf = UnpackBuf::new(&input_bytes);
let trusted: TrustedData<'_, MyData> = MyData::unpack_and_validate(buf)?;

// Cannot access trusted logic until valid
trusted.get(); // Fully safe
```

Guaranteed: Data must be validated before it compiles. No unsafe access is even possible without going through the Validate gate.

## Raw Bytes

Use `ByteBuf<T, N>` to handle fixed-size byte arrays before parsing:

```rust
use briny::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = ByteBuf::<u32, 4>::new(42u32.to_le_bytes());
    let _parsed = input.parse()?; // validated and parsed as u32
    Ok(())
}
```

This is useful when reading from sockets, files, or hardware registers.

### Prelude

`briny` provides a prelude module for ergonomic imports:

```rust
use briny::prelude::*;
// brings in: Validate, TrustedData, UntrustedData, Pack, Unpack, etc.
```

This crate is #![no_std], fully portable, and ideal for embedded and security-critical systems.

## Project Status

- [*] Security-first API design
- [*] 100% safe Rust (no unsafe)
- [*] Fully tested (integration and unit tests)
- [*] No dependencies
- [*] `#![no_std]` support
- [*] Not dependent on `alloc`
- [!] Community audits welcome

## Contributing

Contributions, bug reports, and suggestions are welcome! This project aims to help build verifiably secure foundations for low-level and embedded Rust development.

### License

`briny` is under an MIT license.
