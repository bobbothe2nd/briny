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

This is useful when for casting `NonNull<T>` to `*mut T` or `NonZeroU8` to `u8`, even when `u8` to `NonZeroU8` is invalid.
This system of directional bit-validity took a looong time to get right.

There are also alignment functions:

```rust
let addr = 123;
let align = 16;

let aligned_addr = briny::align::align_up(addr, align);
assert_eq!(aligned_addr, 128);
```

and for advanced users, `briny::raw::cast::reinterpret_unchecked` offers an unchecked `transmute`. It still doesn't work on unsized types (thats what `transmute_copy` is for), but it does work on independently sized types. This goes well with `reinterpret`, a safe alternative to `transmute` for supported types.

## Custom Types

This is the most complex part of `briny`. Luckily for you, the API exposed by it is much simpler:

```rust
#[repr(packed)]
#[derive(briny::traits::Auto)]
struct Pair {
    a: u32,
    b: u64,
}
```

Here, `briny::traits::Auto` is just a re-export of `stable_auto::Auto`. It derives a trait implementation from the struct's fields. This is used to implement all of `briny`'s high-level bitcasting traits through blanket implementations.

Both `a: u32` and `b: u64` are `Pod` types, so it implements `Pod`. However, that isn't enough to use it in bitcasts. That's where the `#[repr(packed)]` comes in. Without that, there would be padding and if you used any functions to cast it, it would fail to compile.

Notice how you don't need to derive `Pod` or `Layout` to cast. You don't even need to depend on `briny`. Just use `stable_auto` if you're defining types. That almost sounds scary, but it's also the most important feature. For most use cases, you never have to touch `briny` internals like `CopySafe` or `Zeroable`. They get implemented by deriving `Auto`.

See this in action:

```rust
use briny::raw::cast;

#[repr(align(16))]
#[derive(briny::traits::Auto)]
struct Abc {
    a: u32,
    b: u64,
    c: [u16; 2],
}

let a: u128 = 1000;
let b: Abc = cast::reinterpret(a);
let a2: u128 = cast::reinterpret(b);
assert_eq!(a, a2);
```

## Errors

The new system of using `stable_auto` causes some very confusing errors to arise:

```text
error[E0277]: the trait bound `bool: traits::__auto_Pod_inner` is not satisfied
   --> src\lib.rs:189:35
    |
189 | let true_bool: bool = reinterpret(1);
    |                       ----------- ^ the trait `traits::__auto_Pod_inner` is not implemented for `bool`
    |                       |
    |                       required by a bound introduced by this call
    |
    = help: the following other types implement trait `traits::__auto_Pod_inner`:
              ()
              Cell<T>
              ManuallyDrop<T>
              MaybeNull<T>
              Option<T>
              PhantomData<T>
              PhantomPinned
              Saturating<T>
            and 32 others
    = note: required for `stable_auto::AllImpl<bool>` to implement `traits::__auto_Pod_inner`
    = note: required for `bool` to implement `traits::__auto_Pod_recursive`
    = note: required for `bool` to implement `Pod`
    = note: required for `{integer}` to implement `briny::traits::Layout<bool>`
note: required by a bound in `reinterpret`
   --> src\raw\cast.rs:17:20
    |
 15 | pub const fn reinterpret<T, U>(src: T) -> U
 16 | where
 17 |     T: NoPadding + Layout<U>,
    |                    ^^^^^^^^^ required by this bound in `reinterpret`
```

This just means you tried to cast `i32` to `bool`. This is invalid, and so it gets rejected.

You can see the code that caused this error here:

```compile_fail
use briny::raw::cast::reinterpret;
let true_bool: bool = reinterpret(1);
assert!(true_bool);
```

## Contributing

Contributions, bug reports, and suggestions are welcome! This project aims to help build verifiably secure foundations for low-level and embedded Rust development.

`briny` is under an MIT license.
