use briny::prelude::*;

// Dummy type for testing
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct Dummy(u8);

impl<const N: usize> Raw<N> for Dummy {
    fn from_bytes(bytes: [u8; N]) -> Result<Self, ValidationError> {
        if N != 1 {
            return Err(ValidationError);
        }
        Ok(Dummy(bytes[0]))
    }

    fn to_bytes(&self) -> [u8; N] {
        [self.0; N]
    }
}

impl Validate for Dummy {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0 == 0xAA {
            Err(ValidationError)
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_chunks_valid() {
    let buf = ByteBuf::<Dummy, 4>::new([1, 2, 3, 4]);
    let chunks = buf.chunks::<Dummy, 1>().unwrap();
    let items: Vec<_> = chunks.collect();
    assert_eq!(items.len(), 4);
    assert_eq!(items[0].as_ref().0, 1);
    assert_eq!(items[3].as_ref().0, 4);
}

#[test]
fn test_chunks_misaligned_returns_err() {
    let buf = ByteBuf::<Dummy, 5>::new([0; 5]);
    let result = buf.chunks::<Dummy, 4>();
    assert!(result.is_err());
}

#[test]
fn test_as_untrusted() {
    let buf = ByteBuf::<Dummy, 1>::new([42]);
    let val = buf.as_untrusted().unwrap();
    assert_eq!(val.as_ref().0, 42);
}

#[test]
fn test_try_unpack_ok() {
    let buf = ByteBuf::<Dummy, 1>::new([1]);
    let val = buf.try_unpack().unwrap();
    assert_eq!(val.0, 1);
}

#[test]
fn test_try_unpack_invalid() {
    let buf = ByteBuf::<Dummy, 1>::new([0xAA]); // invalid
    assert!(buf.try_unpack().is_err());
}

#[test]
fn test_peek_some() {
    let buf = ByteBuf::<Dummy, 1>::new([99]);
    let val = buf.peek::<Dummy, 1>().unwrap();
    assert_eq!(val.as_ref().0, 99);
}

#[test]
fn test_pop_some() {
    let buf = ByteBuf::<Dummy, 2>::new([8, 9]);
    let (first, tail) = buf.pop::<Dummy, 1>().unwrap();
    assert_eq!(first.as_ref().0, 8);
    assert_eq!(tail, &[9]);
}
