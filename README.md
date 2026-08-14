# `briny`

`briny` offers typed casts, aligning functions, and type-level abstraction.

## Usage

Casting

```rust
let a: u8 = 1000;
let b: i8 = briny::raw::cast:::cast(&a);
assert_eq(b, core::mem::transmute::<u8, i8>(a));
```

And you'd be correct to say that looks completely useless. But the best part is that it works on slices too!

Wait... how is this different from bytemuck though?

1. Clean error handling
2. No dependencies
3. Theres more

`briny` isn't just casts, it's alignment and a `MaybeNull` structure too.

`MaybeNull` is like `MaybeUninit` and `Option` combined into one structure. It has it's own way of optimizing zero bitpatterns, using a trait. It has a completely safe API and even has it's own macro to match over null and initialized values.

Alignment is super easy to use in `briny` because it uses an `Unaligned` trait and works at compile time. There are functions to align all unsigned integer types and both immutable and mutable pointers at compile time. The `align` module also defines some constants that can help alignment or just manage memory in general. Particularly different units in measuring memory (byte, GB, GiB, WORD, QWORD, etc.).

## Contributing

Contributions, bug reports, and suggestions are welcome! This project aims to help build verifiably secure foundations for low-level and embedded Rust development.

### License

`briny` is under an MIT license.
