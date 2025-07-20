use briny::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Dummy;

impl Raw<1> for Dummy {
    fn from_bytes(_: [u8; 1]) -> Result<Self, ValidationError> {
        Ok(Dummy)
    }
    fn to_bytes(&self) -> [u8; 1] {
        [0]
    }
}

impl Raw<2> for Dummy {
    fn from_bytes(_: [u8; 2]) -> Result<Self, ValidationError> {
        Ok(Dummy)
    }
    fn to_bytes(&self) -> [u8; 2] {
        [0; 2]
    }
}

impl Raw<4> for Dummy {
    fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
        let _ = bytes;
        Ok(Dummy)
    }
    fn to_bytes(&self) -> [u8; 4] {
        [0; 4] // doesn't matter for tests
    }
}

impl Validate for Dummy {
    fn validate(&self) -> Result<(), briny::trust::ValidationError> {
        Ok(())
    }
}

#[test]
fn test_bytebuf_peek_too_short() {
    let buf = ByteBuf::<Dummy, 2>::new([1, 2]);
    assert!(buf.peek::<Dummy, 4>().is_none());
}

#[test]
fn test_bytebuf_pop_too_short() {
    let buf = ByteBuf::<Dummy, 3>::new([1, 2, 3]);
    assert!(buf.peek::<Dummy, 4>().is_none());
}

#[test]
fn test_chunks_zero_len() {
    let buf = ByteBuf::<Dummy, 0>::new([]);
    let iter = buf.chunks::<Dummy, 1>().unwrap();
    assert_eq!(iter.count(), 0);
}

#[test]
fn test_chunks_exact_match() {
    let buf = ByteBuf::<Dummy, 4>::new([0, 0, 0, 0]);
    let mut iter = buf.chunks::<Dummy, 4>().unwrap();
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
}

#[test]
fn test_chunks_exact_division() {
    let buf = ByteBuf::<u32, 8>::new([0u8; 8]);
    let iter = buf.chunks::<u32, 4>().unwrap();
    assert_eq!(iter.count(), 2);
}

#[test]
fn test_chunks_overflow() {
    // 5 % 2 = 1 — should fail
    let buf = ByteBuf::<Dummy, 5>::new([1, 2, 3, 4, 5]);
    assert!(buf.chunks::<Dummy, 2>().is_err());
}

#[test]
fn test_chunks_multiple_valid_items() {
    let buf = ByteBuf::<Dummy, 6>::new([1, 2, 3, 4, 5, 6]);
    let iter = buf.chunks::<Dummy, 2>().unwrap();
    assert_eq!(iter.count(), 3);
}

#[test]
fn test_chunks_misaligned() {
    let buf = ByteBuf::<u32, 5>::new([0u8; 5]);
    assert!(buf.chunks::<u32, 4>().is_err());
}

#[test]
fn fuzz_like_byte_patterns() {
    for len in 0..16 {
        let mut data = [0u8; 16];
        for i in 0..len {
            data[i] = i as u8;
        }
        let buf = ByteBuf::<Dummy, 16>::new(data);
        let _ = buf.peek::<Dummy, 4>();
        let _ = buf.pop::<Dummy, 4>();
    }
}

#[test]
fn test_validation_must_occur() {
    struct RejectAll;
    impl Validate for RejectAll {
        fn validate(&self) -> Result<(), ValidationError> {
            Err(ValidationError)
        }
    }

    let raw = RejectAll;
    let result = TrustedData::new(raw);
    assert!(result.is_err());
}

#[test]
fn test_try_map_invalidates_bad_transform() {
    struct T(u8);
    impl Validate for T {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.0 == 42 {
                Ok(())
            } else {
                Err(ValidationError)
            }
        }
    }

    let good = TrustedData::new(T(42)).unwrap();

    // Transformation breaks invariants
    let result = good.try_map(|_| T(0));
    assert!(result.is_err());
}

#[test]
fn test_unpack_validation_guard() {
    struct Foo(u8);

    impl Foo {
        fn unpack(input: UnpackBuf<'_>) -> Result<Self, ValidationError> {
            Ok(Foo(input.as_slice()[0]))
        }
    }

    impl Validate for Foo {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.0 == 99 {
                Ok(())
            } else {
                Err(ValidationError)
            }
        }
    }

    impl Unpack for Foo {
        fn unpack_and_validate(
            input: UnpackBuf<'_>,
        ) -> Result<TrustedData<'_, Self>, ValidationError> {
            let raw = Self::unpack(input)?;
            TrustedData::new(raw)
        }
    }

    let valid_buf = [99u8];
    let invalid_buf = [42u8];

    let good = TrustedData::<Foo>::unpack(UnpackBuf::new(&valid_buf));
    let bad = TrustedData::<Foo>::unpack(UnpackBuf::new(&invalid_buf));

    assert!(good.is_ok());
    assert!(bad.is_err());
}

#[test]
fn test_validate_with_context() {
    struct Ctx(u8);
    struct Coded(u8);

    impl Validate<Ctx> for Coded {
        fn validate_with(&self, ctx: &Ctx) -> Result<(), ValidationError> {
            if self.0 == ctx.0 {
                Ok(())
            } else {
                Err(ValidationError)
            }
        }

        fn validate(&self) -> Result<(), ValidationError> {
            Err(ValidationError) // force use of context
        }
    }

    let good = Coded(7);
    let bad = Coded(2);
    let ctx = Ctx(7);

    assert!(good.validate_with(&ctx).is_ok());
    assert!(bad.validate_with(&ctx).is_err());
}
