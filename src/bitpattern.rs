//! A stronger version of `NonNullable`.

use crate::{
    private::Private,
    traits::{InvalidPattern, Layout, NonNullable},
};

/// Compatible with `match_null`:
///
/// ```rust
/// let num = briny::bitpattern::OtherUsize::<123>::new(321).unwrap();
/// let not_pattern = briny::bitpattern::NotPattern::new(num);
///
/// let string = briny::match_null!(
///     match not_pattern {
///         Init(val) => { format!("{val}") }
///         Null => { "invalid".to_string() }
///     }
/// );
///
/// assert_eq!(string, "321");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NotPattern<T: InvalidPattern>(T);

impl<T: InvalidPattern> Private for NotPattern<T> {}

impl<T: InvalidPattern> NotPattern<T> {
    /// Creates a new value.
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        const {
            assert!(
                size_of::<T>() == size_of::<T::Valid>(),
                "valid type of different size than invalid type"
            );
        }

        Self(val)
    }

    /// Creates an invalid value.
    #[inline(always)]
    pub const fn invalid() -> Self {
        const {
            assert!(
                size_of::<T>() == size_of::<T::Valid>(),
                "valid type of different size than invalid type"
            );
        }

        Self(T::INVALID)
    }

    /// Checks for initialization.
    #[inline(always)]
    pub fn is_init(self) -> bool {
        self.0.check_valid()
    }

    /// Checks for initialization.
    #[inline(always)]
    pub fn is_invalid(self) -> bool {
        !self.is_init()
    }

    /// Sets the type to an invalid value
    #[inline(always)]
    pub fn invalidate(&mut self) {
        self.0 = T::INVALID;
    }

    /// Gets the valid value if initialized else `None`.
    #[inline(always)]
    pub fn into_inner(self) -> Option<T::Valid>
    where
        T: Copy,
    {
        if self.is_init() {
            Some(unsafe { self.into_inner_unchecked() })
        } else {
            None
        }
    }

    /// Gets the valid value without checking initialization.
    ///
    /// # Safety
    ///
    /// The type must be valid
    #[inline(always)]
    pub const unsafe fn into_inner_unchecked(self) -> T::Valid {
        unsafe { crate::raw::cast::reinterpret_unchecked(self) }
    }
}

macro_rules! impl_other {
    ($name:ident, $valid:ident) => {
        /// Represents any bitpattern that isnt `INVALID`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name<const INVALID: $valid>(pub(super) $valid);

        unsafe impl NonNullable for $name<0> {}
        unsafe impl<const INVALID: $valid> Layout<$name<INVALID>> for $valid {}

        unsafe impl<const INVALID: $valid> InvalidPattern for $name<INVALID> {
            type Valid = $valid;

            const INVALID: Self = Self(INVALID);

            #[inline(always)]
            fn check_valid(self) -> bool {
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
