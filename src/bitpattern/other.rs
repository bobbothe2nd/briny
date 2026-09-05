use crate::{bitpattern::InvalidPattern, traits::{NonNullable, StableLayout}};

macro_rules! impl_other {
    ($name:ident, $valid:ident) => {
        /// Represents any bitpattern that isnt `INVALID`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name<const INVALID: $valid>(pub(super) $valid);

        unsafe impl NonNullable for $name<0> {}
        unsafe impl<const INVALID: $valid> StableLayout for $name<INVALID> {}

        unsafe impl<const INVALID: $valid> InvalidPattern for $name<INVALID> {
            type Valid = $valid;

            const INVALID: Self = Self(INVALID);

            #[inline(always)]
            fn is_valid(self) -> bool {
                self.0 != INVALID
            }
        }

        impl<const INVALID: $valid> $name<INVALID> {
            /// Creates a new value by checking validity.
            #[inline(always)]
            pub const fn new(val: $valid) -> Option<Self> {
                if val == INVALID {
                    None
                } else {
                    Some(Self(val))
                }
            }

            /// Creates a new value without checking validity.
            #[inline(always)]
            pub const fn new_unchecked(val: $valid) -> Self {
                Self(val)
            }
        }
    };
}

impl_other!(OtherUsize, usize);
impl_other!(OtherU128, u128);
impl_other!(OtherU64, u64);
impl_other!(OtherU32, u32);
impl_other!(OtherU16, u16);
impl_other!(OtherU8, u8);

impl_other!(OtherIsize, isize);
impl_other!(OtherI128, i128);
impl_other!(OtherI64, i64);
impl_other!(OtherI32, i32);
impl_other!(OtherI16, i16);
impl_other!(OtherI8, i8);
