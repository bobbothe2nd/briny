//! A stronger version of `NonNullable`.

mod other;

pub use other::{
    OtherI128, OtherI16, OtherI32, OtherI64, OtherI8, OtherIsize,
    OtherU128, OtherU16, OtherU32, OtherU64, OtherU8, OtherUsize,
};

use crate::{private::Private, traits::StableLayout};

/// Every bitpattern must be valid except the one pattern `is_valid` checks for.
pub unsafe trait InvalidPattern: StableLayout {
    /// The initialized type to coerce to.
    type Valid;

    /// Defines invalid bitpattern as valid.
    const INVALID: Self;

    /// Checks for one invalid bitpattern.
    #[must_use]
    fn is_valid(self) -> bool;
}

/// Compatible with `match_null`:
///
/// ```rust
/// let num = briny::bitpattern::OtherUsize::<123>::new(321).unwrap();
/// let not_pattern = briny::bitpattern::NotPattern::new(num);
///
/// let string = briny::match_null!(
///     match not_pattern {
///         Init(val) => { format!("{val}") }
///         Null => { "null".to_string() }
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
    pub fn new(val: T) -> Self {
        const {
            assert!(
                size_of::<T>() == size_of::<T::Valid>(),
                "valid type of different size than invalid type"
            );
        }

        Self(val)
    }

    /// Creates a null value.
    #[inline(always)]
    pub fn null() -> Self {
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
        self.0.is_valid()
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
    #[inline(always)]
    pub unsafe fn into_inner_unchecked(self) -> T::Valid {
        unsafe {
            crate::raw::cast::reinterpret_unchecked(self)
        }
    }
}
