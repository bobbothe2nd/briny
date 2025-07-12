use briny::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Str8([u8; 8]);

impl Raw<8> for Str8 {
    fn from_bytes(bytes: [u8; 8]) -> Result<Self, ValidationError> {
        core::str::from_utf8(&bytes).map_err(|_| ValidationError)?;
        Ok(Str8(bytes))
    }

    fn to_bytes(&self) -> [u8; 8] {
        self.0
    }
}

impl Validate for Str8 {
    fn validate(&self) -> Result<(), ValidationError> {
        core::str::from_utf8(&self.0).map_err(|_| ValidationError)?;
        Ok(())
    }
}

#[test]
fn from_valid_str_to_bytes() {
    let b: ByteBuf<Str8, 8> = ByteBuf::from_str("hello").unwrap();
    let bytes = b.as_bytes();
    assert_eq!(&bytes[..5], b"hello");
    assert_eq!(&bytes[5..], &[0; 3]); // padded with 0s
}
