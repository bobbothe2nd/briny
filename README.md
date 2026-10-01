# `briny`

`briny` offers typed casts, aligning functions, and type-level abstraction. A collection of small, often re-implemented functions, traits, and constants.

## Usage

Casting:

```rust
let a: u16 = 1000;
let b: &i16 = briny::raw::cast::cast(&a);
assert_eq!(*b, a.cast_signed());
```

- `copy` for automatically copying the casted type
- `to_bytes`/`from_bytes` for converting types to bytes.

And you'd be correct to say that looks completely useless. But the best part is that it works on slices too!

Rather than competing with `bytemuck`, `briny` complements it. This includes traits like `Layout` that have a less restricting invariant of:

- `T: Layout<U>`: It's safe to transmute type `T` to type `U` but not `U` to `T`, `T` to `_`, or `_` to `T`.

There are also alignment functions:

```rust
let addr = 123;
let align = 16;

let aligned_addr = briny::align::align_up(addr, align);
assert_eq!(aligned_addr, 128);
```

and for advanced users, `briny::raw::cast::reinterpret_unchecked` offers an unchecked `transmute`. It still doesn't work on unsized types (thats what `transmute_copy` is for), but it does work on independently sized types. This goes well with `reinterpret`, a safe alternative to `transmute` for supported types.

## Contributing

Contributions, bug reports, and suggestions are welcome! This project aims to help build verifiably secure foundations for low-level and embedded Rust development.

`briny` is under an MIT license.
