# `briny`

`briny` offers typed casts, aligning functions, and type-level abstraction. A collection of small, often re-implemented functions, traits, and constants.

## Usage

Casting

```rust
let a: u8 = 1000;
let b: i8 = briny::raw::cast:::cast(&a);
assert_eq(b, core::mem::transmute::<u8, i8>(a));
```

And you'd be correct to say that looks completely useless. But the best part is that it works on slices too!

Rather than competing with `bytemuck`, `briny` complements it. This includes traits like `Layout` that say its safe to transmute type `T` to type `U` but not `U` to `T`, `T` to `_`, or `_` to `T`.

There are also alignment functions:

```rust
let addr = 123;
let align = 16;

let aligned_addr = briny::align::align_up(addr, align);
assert_eq!(aligned_addr, 128);
```

and as of `v0.8.2`, the ZTA traits have been re-implemented which have been removed since `0.3.0`. They will probably receive no major update soon.

## Contributing

Contributions, bug reports, and suggestions are welcome! This project aims to help build verifiably secure foundations for low-level and embedded Rust development.

`briny` is under an MIT license.
